use async_nats::Client;
use async_nats::subject::ToSubject;
use serde::Serialize;
use crate::cableway::send_message;

pub mod calendar;

#[derive(Serialize)]
struct EventEnvelope<T : Serialize> {
    #[serde(rename = "type")]
    pub event: String,
    pub data: T,
}

pub async fn send_event<T : Serialize>(
    cableway: &Client,
    event: &str,
    subject: impl ToSubject,
    data: T
) -> Result<(), async_nats::Error> {
    let obj = EventEnvelope {
        event: event.to_string(),
        data
    };
    send_message(cableway, subject, obj).await
}