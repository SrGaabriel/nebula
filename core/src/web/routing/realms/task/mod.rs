use crate::app::NebulaApp;
use crate::data::snowflake::Snowflake;
use crate::schema::{realm_tasks, users};
use crate::service::snowflake::next_snowflake;
use crate::web::routing::dto::TaskDto;
use crate::web::routing::error::{ok, NebulaResponse};
use crate::web::routing::middlewares::validation::ValidJson;
use axum::extract::{Path, State};
use axum::Extension;
use garde::Validate;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateTaskRequest {
    #[garde(length(min = 1, max = 48))]
    pub title: String,
    #[garde(length(max = 2048))]
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