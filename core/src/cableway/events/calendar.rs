use async_nats::Client;
use serde::{Deserialize, Serialize};
use crate::cableway::send_message;
use crate::data::snowflake::Snowflake;
use crate::web::routing::dto::RealmEventDto;

#[derive(Serialize, Deserialize)]
struct CalendarEventCreated {
    pub event: RealmEventDto
}

pub async fn send_event_created(
    cableway: &Client,
    event: RealmEventDto
) -> Result<(), async_nats::Error> {
    let message = CalendarEventCreated { event };
    send_message(cableway, format!("realm.{}.calendar.event_created", message.event.realm_id), message).await
}

#[derive(Serialize, Deserialize)]
struct CalendarEventDeleted {
    pub event_id: Snowflake
}

pub async fn send_event_deleted(
    cableway: &Client,
    event_id: Snowflake
) -> Result<(), async_nats::Error> {
    let message = CalendarEventDeleted { event_id };
    send_message(cableway, format!("realm.{}.calendar.event_deleted", message.event_id), message).await
}