use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::data::snowflake::Snowflake;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "realms")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Snowflake,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Snowflake,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::OwnerId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Owner,
    #[sea_orm(
        has_many = "super::realm_members::Entity",
        from = "Column::Id",
        to = "super::realm_members::Column::RealmId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    RealmMembers,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl Related<super::realm_members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RealmMembers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}