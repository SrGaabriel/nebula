use serde::{Deserialize, Serialize};
use crate::data::snowflake::Snowflake;
use crate::schema::users;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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