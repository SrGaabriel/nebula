use crate::util::validation::is_sane;
use crate::app::NebulaApp;
use crate::data::snowflake::Snowflake;
use crate::schema::{realm_tasks, users};
use crate::service::snowflake::next_snowflake;
use crate::web::routing::dto::TaskDto;
use crate::web::routing::error::{ok, NebulaResponse};
use crate::web::routing::middlewares::validation::{ValidJson, ValidQuery};
use axum::extract::{Path, State};
use axum::Extension;
use garde::Validate;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use sea_query::{Condition, ExprTrait};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateTaskRequest {
    #[garde(length(min = 1, max = 48), custom(is_sane))]
    pub title: String,
    #[garde(length(max = 2048), inner(custom(is_sane)))]
    pub description: Option<String>,
    #[garde(skip)]
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    #[garde(skip)]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    #[garde(skip)]
    pub planned_for: Option<chrono::DateTime<chrono::Utc>>,
    #[garde(range(min = 0, max = 2))]
    pub priority: Option<u8>,
    #[garde(skip)]
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskObject {
    pub task: TaskDto
}

pub async fn create_task(
    Path(realm_id): Path<Snowflake>,
    Extension(user): Extension<users::Model>,
    State(app): State<NebulaApp>,
    ValidJson(payload): ValidJson<CreateTaskRequest>
) -> NebulaResponse<TaskObject> {
    let priority = match payload.priority {
        Some(0) => Some(realm_tasks::Priority::Discardable),
        Some(1) => Some(realm_tasks::Priority::Desirable),
        Some(2) => Some(realm_tasks::Priority::Important),
        _ => None
    };

    let task_id = next_snowflake();
    let new_task = realm_tasks::ActiveModel {
        id: Set(task_id),
        realm_id: Set(realm_id),
        author_id: Set(user.id),
        title: Set(payload.title),
        description: Set(payload.description),
        priority: Set(priority),
        due_date: Set(payload.due_date),
        start_date: Set(payload.start_date),
        planned_for: Set(payload.planned_for),
        completed: Set(payload.completed),
        updated_at: Set(chrono::Utc::now().naive_utc())
    };
    let inserted_task = new_task.insert(&app.db)
        .await
        .expect("Failed to insert new task");
    let task_dto = TaskDto::from_model(inserted_task);
    ok(TaskObject { task: task_dto })
}

#[derive(Deserialize, Debug, Validate)]
pub struct TaskQuery {
    #[garde(skip)]
    pub completed: Option<bool>,
    #[garde(inner(inner(range(min = 0, max = 2))))]
    pub priorities: Option<Vec<u8>>,
    #[garde(skip)]
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    #[garde(skip)]
    pub to: Option<chrono::DateTime<chrono::Utc>>
}

pub async fn get_tasks(
    Path(realm_id): Path<Snowflake>,
    Extension(_user): Extension<users::Model>,
    State(app): State<NebulaApp>,
    ValidQuery(query): ValidQuery<TaskQuery>
) -> NebulaResponse<Vec<TaskDto>> {
    let mut task_query = realm_tasks::Entity::find()
        .filter(realm_tasks::Column::RealmId.eq(realm_id));
    if let Some(completed) = query.completed {
        task_query = task_query.filter(realm_tasks::Column::Completed.eq(completed));
    }
    if let Some(priority) = query.priorities {
        let condition = priority.iter().fold(Condition::any(), |cond, p| {
            match p {
                0 => cond.add(realm_tasks::Column::Priority.eq(realm_tasks::Priority::Discardable)),
                1 => cond.add(realm_tasks::Column::Priority.eq(realm_tasks::Priority::Desirable)),
                2 => cond.add(realm_tasks::Column::Priority.eq(realm_tasks::Priority::Important)),
                _ => cond,
            }
        });
        task_query = task_query.filter(condition);
    }
    if let Some(from) = query.from {
        task_query = task_query.filter(
            realm_tasks::Column::DueDate.gte(from)
                .or(realm_tasks::Column::StartDate.gte(from))
                .or(realm_tasks::Column::PlannedFor.gte(from))
        );
    }
    if let Some(to) = query.to {
        task_query = task_query.filter(
            realm_tasks::Column::DueDate.lte(to)
                .or(realm_tasks::Column::StartDate.lte(to))
                .or(realm_tasks::Column::PlannedFor.lte(to))
        );
    }
    let tasks = task_query
        .all(&app.db)
        .await
        .expect("Failed to query tasks");

    let task_dtos: Vec<TaskDto> = tasks.into_iter().map(TaskDto::from_model).collect();
    ok(task_dtos)
}