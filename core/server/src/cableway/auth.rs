use async_nats::Client;
use async_std::prelude::StreamExt;
use sea_orm::DatabaseConnection;
use crate::app::AppConfig;
use crate::data::snowflake::Snowflake;

#[derive(serde::Serialize, Debug)]
pub struct AuthResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Snowflake>,
}

pub async fn open_authentication_channel(config: &AppConfig, client: &Client, db: &DatabaseConnection) {
    let subscriber = client.subscribe("internal.validate_token").await;
    let mut subscriber = subscriber.expect("Failed to subscribe to internal.auth channel");

    // todo: maybe use an arc here
    let config = config.clone();
    let db = db.clone();
    let client = client.clone();
    tokio::spawn(async move {
        while let Some(message) = subscriber.next().await {
            if let Some(reply_to) = message.reply {
                let token = String::from_utf8(message.payload.to_vec())
                    .unwrap_or_else(|_| "".to_string());
                let auth_result = crate::service::auth::authenticate(&config, &db, token).await;
                let response = if auth_result.is_ok() {
                    AuthResponse {
                        success: true,
                        user_id: Some(auth_result.unwrap().id),
                    }
                } else {
                    AuthResponse {
                        success: false,
                        user_id: None,
                    }
                };
                let bytes = serde_json::to_vec(&response)
                    .expect("Failed to serialize auth response");

                client.publish(reply_to, bytes.into())
                    .await
                    .expect("Failed to send auth response");
            }
        }
    });
}