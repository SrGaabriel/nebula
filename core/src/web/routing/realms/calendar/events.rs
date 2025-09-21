use axum::{Extension, Json};
use axum::extract::{Path, State};
use chrono::{DateTime, Utc};
use sea_orm::{EntityTrait, Set};
use crate::app::NebulaApp;
use crate::data::calendar::RecurrenceRule;
use crate::data::snowflake::Snowflake;
use crate::schema::users;
use crate::schema::realm_events;
use crate::service::snowflake::next_snowflake;
use crate::web::routing::dto::RealmEventDto;
use crate::web::routing::error::{error, ok, NebulaResponse};
use crate::web::routing::realms::calendar::RealmEventObject;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateEventRequest {
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub recurrence: Option<RecurrenceRule>
}

pub async fn create_event(
    Path(realm_id): Path<Snowflake>,
    Extension(user): Extension<users::Model>,
    State(app): State<NebulaApp>,
    Json(payload): Json<CreateEventRequest>
) -> NebulaResponse<RealmEventObject> {
    let db = &app.state.read().await.db;
    let encoded_recurrence = match &payload.recurrence {
        Some(rule) => {
            let encoded = rule.to_u64();
            if encoded.is_err() {
                return error(
                    axum::http::StatusCode::BAD_REQUEST,
                    "Invalid recurrence rule (too big?)"
                );
            }
            Some(encoded.unwrap() as i64)
        }
        None => None
    };

    let snowflake = next_snowflake();
    let event = realm_events::ActiveModel {
        id: Set(snowflake),
        name: Set(payload.name.clone()),
        description: Set(payload.description.clone()),
        location: Set(payload.location.clone()),
        created_by: Set(user.id),
        realm_id: Set(realm_id),
        start_time: Set(payload.start_time),
        end_time: Set(payload.end_time),
        recurrence: Set(encoded_recurrence),
    };
    realm_events::Entity::insert(event)
        .exec(db)
        .await
        .expect("Failed to insert event");

    ok(RealmEventObject {
        event: RealmEventDto {
            id: snowflake,
            name: payload.name,
            description: payload.description,
            location: payload.location,
            created_by: user.id,
            realm_id,
            start_time: payload.start_time,
            end_time: payload.end_time,
            recurrence: payload.recurrence
        }
    })
}