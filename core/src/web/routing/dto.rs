use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    pub id: u64,
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
    pub id: u64,
    pub name: String,
    pub owner_id: u64
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