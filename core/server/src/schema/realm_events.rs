use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;
use sea_orm::DerivePrimaryKey;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, Related, RelationTrait};
use serde::{Deserialize, Serialize};
use crate::data::snowflake::Snowflake;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "realm_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Snowflake,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub created_by: Snowflake,
    pub realm_id: Snowflake,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub recurrence: Option<String>
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::CreatedBy",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Creator,
    #[sea_orm(
        belongs_to = "super::realms::Entity",
        from = "Column::RealmId",
        to = "super::realms::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Realm,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Creator.def()
    }
}

impl Related<super::realms::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Realm.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}