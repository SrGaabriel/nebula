pub mod events;

use async_nats::{Client, ServerAddr};
use crate::app::AppConfig;

pub async fn connect(config: &AppConfig) -> Client {
    async_nats::connect(config.cableway_addr.to_string().parse::<ServerAddr>().unwrap())
        .await
        .expect("Failed to connect to cableway")
}