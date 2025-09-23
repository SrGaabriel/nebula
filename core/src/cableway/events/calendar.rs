use async_nats::Client;
use async_nats::subject::ToSubject;
use serde::{Deserialize, Serialize};
use crate::web::routing::dto::RealmEventDto;

#[derive(Serialize, Deserialize)]
pub struct CalendarEventCreated {
    pub event: RealmEventDto
}

pub async fn send_event_created(
    cableway: &Client,
    event: RealmEventDto
) -> Result<(), async_nats::Error> {
    let message = CalendarEventCreated { event };
    send(cableway, format!("realm.{}.calendar.event_created", message.event.realm_id), message).await
}

async fn send<T : Serialize>(
    cableway: &Client,
    subject: impl ToSubject,
    data: T
) -> Result<(), async_nats::Error> {
    let payload = serde_json::to_vec(&data).expect("Failed to serialize message");
    cableway.publish(subject, payload.into()).await?;
    cableway.flush().await?;
    Ok(())
}