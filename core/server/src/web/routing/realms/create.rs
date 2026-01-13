use crate::app::NebulaApp;
use crate::schema::{realms, users};
use crate::service::snowflake::next_snowflake;
use crate::util::validation::is_sane;
use crate::web::routing::dto::RealmDto;
use crate::web::routing::error::{error, ok, NebulaResponse};
use crate::web::routing::middlewares::validation::ValidJson;
use crate::web::routing::realms::RealmObject;
use axum::extract::State;
use axum::Extension;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::service;

#[derive(serde::Serialize, serde::Deserialize, Debug, garde::Validate)]
pub struct CreateRealmPayload {
    #[garde(length(min = 3, max = 48), custom(is_sane))]
    pub name: String,
    #[serde(default)]
    #[garde(length(max = 1024), inner(custom(is_sane)))]
    pub description: Option<String>,
}

pub async fn create_realm(
    State(app): State<NebulaApp>,
    Extension(user): Extension<users::Model>,
    ValidJson(payload): ValidJson<CreateRealmPayload>
) -> NebulaResponse<RealmObject> {
    let db = &app.db;
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

    let inserted_realm = service::realm::create_realm(db, user.id, new_realm)
        .await
        .expect("Failed to create a new realm");

    let dto = RealmDto::from_model(&inserted_realm);
    ok(RealmObject { realm: dto })
}