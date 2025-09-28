use sea_orm::{DeriveActiveEnum, EntityTrait};
use sea_orm::PrimaryKeyTrait;
use sea_orm::DerivePrimaryKey;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, Related, RelationTrait};
use serde::{Deserialize, Serialize};
use crate::data::snowflake::Snowflake;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "realm_tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Snowflake,
    pub realm_id: Snowflake,
    pub author_id: Snowflake,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub planned_for: Option<chrono::DateTime<chrono::Utc>>,
    pub completed: bool,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(EnumIter, DeriveActiveEnum, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum Priority {
    #[sea_orm(num_value = 0)]
    Discardable,
    #[sea_orm(num_value = 1)]
    Desirable,
    #[sea_orm(num_value = 2)]
    Important
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::realms::Entity",
        from = "Column::RealmId",
        to = "super::realms::Column::Id"
    )]
    Realm,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::AuthorId",
        to = "super::users::Column::Id"
    )]
    Author,
}

impl Related<super::realms::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Realm.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Author.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}