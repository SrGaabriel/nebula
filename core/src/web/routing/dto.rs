use serde::{Deserialize, Serialize};
use crate::data::calendar::RecurrenceRule;
use crate::data::snowflake::Snowflake;
use crate::schema::users;

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
    pub owner_id: Snowflake
}

impl RealmDto {
    pub fn from_model(model: &crate::schema::realms::Model) -> Self {
        RealmDto {
            id: model.id,
            name: model.name.clone(),
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
    pub recurrence: Option<RecurrenceRule>
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
            recurrence: model.recurrence.map(|r| RecurrenceRule::from_u64(r as u64)).transpose().unwrap_or(None)
        }
    }
}