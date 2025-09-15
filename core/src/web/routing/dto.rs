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
            id: model.id as u64,
            name: model.name.clone()
        }
    }
}