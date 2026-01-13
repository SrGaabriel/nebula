use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use crate::data::permissions::{BitwisePermissions, RealmPermissions};
use crate::data::snowflake::Snowflake;
use crate::schema::{realm_members, realms};
use crate::service::snowflake::next_snowflake;

pub async fn create_realm(
    db: &DatabaseConnection,
    user_id: Snowflake,
    new_realm: realms::ActiveModel
) -> Result<realms::Model, sea_orm::DbErr> {
    let inserted_realm = new_realm.insert(db)
        .await?;

    let membership_snowflake = next_snowflake();
    let permissions = RealmPermissions::all();
    let new_membership = realm_members::ActiveModel {
        id: Set(membership_snowflake),
        realm_id: Set(inserted_realm.id),
        user_id: Set(user_id),
        permissions: Set(permissions.bits())
    };

    new_membership.insert(db)
        .await?;
    Ok(inserted_realm)
}