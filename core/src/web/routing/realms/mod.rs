pub mod create;

use axum::Extension;
use axum::extract::{Path, State};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use crate::app::NebulaApp;
use crate::data::snowflake::Snowflake;
use crate::schema::{realm_members, realms, users};
use crate::web::routing::dto::RealmDto;
use crate::web::routing::error::{error, ok, NebulaResponse};

#[derive(Serialize, Deserialize, Debug)]
pub struct RealmObject {
    pub realm: RealmDto
}

pub async fn get_realm(
    State(app): State<NebulaApp>,
    Extension(user): Extension<users::Model>,
    Path(realm_id): Path<Snowflake>
) -> NebulaResponse<RealmObject> {
    let db = &app.state.read().await.db;
    let realm = realms::Entity::find_by_id(realm_id)
        .one(db)
        .await
        .expect("Failed to query realm");

    if realm.is_none() {
        return error(
            axum::http::StatusCode::NOT_FOUND,
            "Realm not found"
        )
    }
    let realm = realm.unwrap();

    let realm_membership = realm_members::Entity::find_membership(
        db,
        realm_id,
        user.id
    ).await.expect("Failed to query realm membership");
    if realm_membership.is_none() {
        return error(
            axum::http::StatusCode::FORBIDDEN,
            "You are not a member of this realm"
        )
    }

    let dto = RealmDto::from_model(&realm);
    ok(RealmObject { realm: dto })
}