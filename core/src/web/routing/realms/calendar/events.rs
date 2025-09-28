use crate::app::NebulaApp;
use crate::cableway::events::calendar::{send_event_created, send_event_deleted};
use crate::data::snowflake::Snowflake;
use crate::schema::realm_events;
use crate::schema::users;
use crate::service::snowflake::next_snowflake;
use crate::util::validation::is_sane;
use crate::web::routing::dto::RealmEventDto;
use crate::web::routing::error::{error, no_content, ok, NebulaResponse};
use crate::web::routing::middlewares::validation::ValidJson;
use crate::web::routing::realms::calendar::RealmEventObject;
use axum::extract::{Path, State};
use axum::Extension;
use chrono::{DateTime, Utc};
use rrule::{RRule, Unvalidated};
use sea_orm::{EntityTrait, Set};

#[derive(serde::Deserialize, serde::Serialize, Debug, garde::Validate)]
pub struct CreateEventRequest {
    #[garde(length(min = 2, max = 48), custom(is_sane))]
    pub name: String,
    #[garde(length(max = 4096), inner(custom(is_sane)))]
    pub description: Option<String>,
    #[garde(length(max = 4096), inner(custom(is_sane)))]
    pub location: Option<String>,
    #[garde(skip)]
    pub start_time: DateTime<Utc>,
    #[garde(skip)]
    pub end_time: Option<DateTime<Utc>>,
    #[garde(skip)]
    pub recurrence: Option<RRule<Unvalidated>>
}

pub async fn create_event(
    Path(realm_id): Path<Snowflake>,
    Extension(user): Extension<users::Model>,
    State(app): State<NebulaApp>,
    ValidJson(payload): ValidJson<CreateEventRequest>
) -> NebulaResponse<RealmEventObject> {
    let db = &app.db;
    let encoded_recurrence = payload.recurrence.as_ref().map(|r| r.to_string());

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
    let dto = RealmEventDto {
        id: snowflake,
        name: payload.name.clone(),
        description: payload.description.clone(),
        location: payload.location.clone(),
        created_by: user.id,
        realm_id,
        start_time: payload.start_time,
        end_time: payload.end_time,
        recurrence: payload.recurrence
    };

    send_event_created(
        &app.cableway,
        dto.clone()
    )
        .await
        .expect("Failed to send event created message");

    ok(RealmEventObject {
        event: dto
    })
}

pub async fn delete_event(
    Path((realm_id, event_id)): Path<(Snowflake, Snowflake)>,
    Extension(_user): Extension<users::Model>,
    State(app): State<NebulaApp>
) -> NebulaResponse<()> {
    let db = &app.db;
    let event = realm_events::Entity::find_by_id(event_id)
        .one(db)
        .await
        .expect("Failed to query event");

    if event.is_none() {
        return error(axum::http::StatusCode::NOT_FOUND, "Event not found");
    }

    let event = event.unwrap();
    if event.realm_id != realm_id {
        return error(axum::http::StatusCode::BAD_REQUEST, "Event does not belong to the specified realm");
    }

    send_event_deleted(
        &app.cableway,
        event_id,
    )
        .await
        .expect("Failed to send event deleted");

    realm_events::Entity::delete_by_id(event_id)
        .exec(db)
        .await
        .expect("Failed to delete event");

    no_content()
}