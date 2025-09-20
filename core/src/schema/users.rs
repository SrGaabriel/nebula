use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::data::snowflake::Snowflake;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Snowflake,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::realms::Entity")]
    Realms,
}

impl Related<super::realms::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Realms.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}