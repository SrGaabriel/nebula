use axum::Extension;
use axum::extract::State;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, QueryFilter, ColumnTrait};
use crate::app::NebulaApp;
use crate::data::permissions::{BitwisePermissions, RealmPermissions};
use crate::schema::{realm_members, realms, users};
use crate::service::snowflake::next_snowflake;
use crate::web::routing::dto::RealmDto;
use crate::web::routing::error::{error, ok, NebulaResponse};
use crate::web::routing::realms::RealmObject;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CreateRealmPayload {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

pub async fn create_realm(
    State(app): State<NebulaApp>,
    Extension(user): Extension<users::Model>,
    axum::Json(payload): axum::Json<CreateRealmPayload>
) -> NebulaResponse<RealmObject> {
    let db = &app.state.read().await.db;

    let existing_realm = realms::Entity::find()
        .filter(realms::Column::Name.eq(payload.name.clone()))
        .filter(realms::Column::OwnerId.eq(user.id))
        .one(db)
        .await
        .expect("Failed to query realms");

    if existing_realm.is_some() {
        return error(
            axum::http::StatusCode::CONFLICT,
            "You already have a realm with the same name"
        );
    }

    let new_realm_snowflake = next_snowflake();
    let new_realm = realms::ActiveModel {
        id: Set(new_realm_snowflake),
        name: Set(payload.name.clone()),
        owner_id: Set(user.id),
        description: Set(payload.description.clone())
    };

    let inserted_realm = new_realm.insert(db)
        .await
        .expect("Failed to insert new realm");

    let membership_snowflake = next_snowflake();
    let permissions = RealmPermissions::all();
    let new_membership = realm_members::ActiveModel {
        id: Set(membership_snowflake),
        realm_id: Set(inserted_realm.id),
        user_id: Set(user.id),
        permissions: Set(permissions.bits())
    };

    new_membership.insert(db)
        .await
        .expect("Failed to insert realm membership");

    let dto = RealmDto::from_model(&inserted_realm);
    ok(RealmObject { realm: dto })
}