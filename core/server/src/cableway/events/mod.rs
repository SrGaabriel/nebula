use async_nats::Client;
use async_nats::subject::ToSubject;
use serde::Serialize;

pub mod calendar;

#[derive(Serialize)]
struct EventEnvelope<T : Serialize> {
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
    let payload = serde_json::to_vec(&obj).expect("Failed to serialize message");
    cableway.publish(subject, payload.into()).await?;
    cableway.flush().await?;
    Ok(())
}