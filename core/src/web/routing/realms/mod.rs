pub mod create;
mod calendar;

use axum::Extension;
use axum::extract::{Path, State};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use crate::app::NebulaApp;
use crate::data::snowflake::Snowflake;
use crate::schema::{realm_members, realms, users};
use crate::web::routing::dto::RealmDto;
use crate::web::routing::error::{error, ok, NebulaResponse};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RealmObject {
    pub realm: RealmDto
}

pub async fn get_realm(
    State(_app): State<NebulaApp>,
    Extension(_user): Extension<users::Model>,
    Extension(realm): Extension<realms::Model>,
    Extension(_realm_membership): Extension<realm_members::Model>,
    Path(_realm_id): Path<Snowflake>
) -> NebulaResponse<RealmObject> {
    let dto = RealmDto::from_model(&realm);
    ok(RealmObject { realm: dto })
}