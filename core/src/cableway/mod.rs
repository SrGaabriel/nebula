pub mod events;

use async_nats::{Client, ServerAddr};
use async_nats::subject::ToSubject;
use serde::Serialize;
use crate::app::AppConfig;

pub async fn connect(config: &AppConfig) -> Client {
    async_nats::connect(config.cableway_addr.to_string().parse::<ServerAddr>().unwrap())
        .await
        .expect("Failed to connect to cableway")
}

async fn send_message<T : Serialize>(
    cableway: &Client,
    subject: impl ToSubject,
    data: T
) -> Result<(), async_nats::Error> {
    let payload = serde_json::to_vec(&data).expect("Failed to serialize message");
    cableway.publish(subject, payload.into()).await?;
    cableway.flush().await?;
    Ok(())
}