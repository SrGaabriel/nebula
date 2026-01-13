use crate::data::snowflake::Snowflake;
use crate::schema::{realm_tasks, users};
use rrule::{RRule, Unvalidated};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserDto {
    pub id: Snowflake,
    pub name: String
}

impl UserDto {
    pub fn from_model(model: &users::Model) -> Self {
        UserDto {
            id: model.id,
            name: model.name.clone()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RealmDto {
    pub id: Snowflake,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Snowflake
}

impl RealmDto {
    pub fn from_model(model: &crate::schema::realms::Model) -> Self {
        RealmDto {
            id: model.id,
            name: model.name.clone(),
            description: model.description.clone(),
            owner_id: model.owner_id
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RealmEventDto {
    pub id: Snowflake,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub created_by: Snowflake,
    pub realm_id: Snowflake,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub recurrence: Option<RRule<Unvalidated>>
}

impl RealmEventDto {
    pub fn from_model(model: &crate::schema::realm_events::Model) -> Self {
        RealmEventDto {
            id: model.id,
            name: model.name.clone(),
            description: model.description.clone(),
            location: model.location.clone(),
            created_by: model.created_by,
            realm_id: model.realm_id,
            start_time: model.start_time,
            end_time: model.end_time,
            recurrence: model.recurrence.as_ref().map(|r| r.parse().unwrap())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RealmEventOccurrenceDto {
    pub event_index: i8,
    pub occurrence_start: chrono::DateTime<chrono::Utc>,
    pub occurrence_end: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RealmScheduleDto {
    pub events: Vec<RealmEventDto>,
    pub tasks: Vec<TaskDto>,
    pub occurrences: Vec<RealmEventOccurrenceDto>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDto {
    pub id: Snowflake,
    pub title: String,
    pub description: Option<String>,
    pub created_by: Snowflake,
    pub realm_id: Snowflake,
    pub priority: Option<u8>,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub planned_for: Option<chrono::DateTime<chrono::Utc>>,
    pub completed: bool,
}

impl TaskDto {
    pub fn from_model(model: realm_tasks::Model) -> Self {
        let priority = match model.priority {
            Some(realm_tasks::Priority::Discardable) => Some(0),
            Some(realm_tasks::Priority::Desirable) => Some(1),
            Some(realm_tasks::Priority::Important) => Some(2),
            None => None
        };

        Self {
            id: model.id,
            title: model.title,
            description: model.description,
            created_by: model.author_id,
            realm_id: model.realm_id,
            priority,
            due_date: model.due_date,
            start_date: model.start_date,
            planned_for: model.planned_for,
            completed: model.completed
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelfStatusDto {
    pub realms: Vec<RealmDto>,
    pub me: UserDto
}