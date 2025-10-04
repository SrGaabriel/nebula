pub mod create;
pub mod calendar;
pub mod task;

use crate::app::NebulaApp;
use crate::schema::realms;
use crate::web::routing::dto::RealmDto;
use crate::web::routing::error::{ok, NebulaResponse};
use axum::extract::State;
use axum::Extension;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RealmObject {
    pub realm: RealmDto
}

pub async fn get_realm(
    State(_app): State<NebulaApp>,
    Extension(realm): Extension<realms::Model>,
) -> NebulaResponse<RealmObject> {
    let dto = RealmDto::from_model(&realm);
    ok(RealmObject { realm: dto })
}