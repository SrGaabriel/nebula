use axum::Extension;
use axum::extract::{Path, State};
use crate::app::NebulaApp;
use crate::data::calendar::RecurrenceRule;
use crate::data::snowflake::Snowflake;
use crate::schema::users;
use crate::web::routing::error::NebulaResponse;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateEventRequest {
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub recurrence: Option<RecurrenceRule>
}

pub async fn create_event(
    Path(realm_id): Path<Snowflake>,
    Extension(user): Extension<users::Model>,
    State(app): State<NebulaApp>
) -> NebulaResponse<()> {
    todo!()
}