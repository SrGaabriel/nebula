use sea_orm::PaginatorTrait;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QuerySelect, RelationTrait, ColumnTrait, QueryFilter};
use sea_orm::{PrimaryKeyTrait, Related};
use sea_orm::DerivePrimaryKey;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use sea_query::Condition;
use serde::{Deserialize, Serialize};
use crate::data::snowflake::Snowflake;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "realm_members")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Snowflake,
    pub realm_id: Snowflake,
    pub user_id: Snowflake,
    pub permissions: i16
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::realms::Entity",
        from = "Column::RealmId",
        to = "super::realms::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Realm,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::realms::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Realm.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub async fn find_membership(
        db: &DatabaseConnection,
        realm_id: Snowflake,
        user_id: Snowflake,
    ) -> Result<Option<Model>, DbErr> {
        Self::find()
            .filter(
                Condition::all()
                    .add(Column::RealmId.eq(realm_id))
                    .add(Column::UserId.eq(user_id))
            )
            .one(db)
            .await
    }

    pub async fn user_has_realm_access(
        db: &DatabaseConnection,
        realm_id: Snowflake,
        user_id: Snowflake,
    ) -> Result<bool, DbErr> {
        let count = Self::find()
            .filter(
                Condition::all()
                    .add(Column::RealmId.eq(realm_id))
                    .add(Column::UserId.eq(user_id))
            )
            .count(db)
            .await?;

        Ok(count > 0)
    }

    pub async fn get_user_permissions(
        db: &DatabaseConnection,
        realm_id: Snowflake,
        user_id: Snowflake,
    ) -> Result<Option<u8>, DbErr> {
        Self::find()
            .select_only()
            .column(Column::Permissions)
            .filter(
                Condition::all()
                    .add(Column::RealmId.eq(realm_id))
                    .add(Column::UserId.eq(user_id))
            )
            .into_tuple::<u8>()
            .one(db)
            .await
    }
}