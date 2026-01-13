use crate::app::NebulaApp;
use crate::data::snowflake::Snowflake;
use crate::service;
use crate::web::routing::dto::RealmScheduleDto;
use crate::web::routing::error::{ok, NebulaResponse};
use crate::web::routing::middlewares::validation::ValidQuery;
use axum::extract::{Path, State};

#[derive(serde::Deserialize, serde::Serialize, Debug, garde::Validate)]
pub struct OccurrenceQuery {
    #[garde(skip)] // todo: implement garde datetime validation
    pub start: chrono::DateTime<chrono::Utc>,
    #[garde(skip)]
    pub end: chrono::DateTime<chrono::Utc>,
}

pub async fn get_occurrences(
    Path(realm_id): Path<Snowflake>,
    State(app): State<NebulaApp>,
    ValidQuery(query): ValidQuery<OccurrenceQuery>
) -> NebulaResponse<RealmScheduleDto> {
    let schedule = service::schedule::get_realm_schedule(
        &app.db,
        realm_id,
        query.start,
        query.end
    )
        .await
        .expect("Failed to get realm schedule");

    ok(schedule)
}