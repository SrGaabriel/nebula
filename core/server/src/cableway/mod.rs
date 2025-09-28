pub mod events;
mod auth;

use async_nats::{Client, ServerAddr};
use async_nats::subject::ToSubject;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use crate::app::AppConfig;

pub async fn start(config: &AppConfig, db: &DatabaseConnection) -> Client {
    let client = connect(config).await;
    auth::open_authentication_channel(config, &client, &db).await;
    client
}

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