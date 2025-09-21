use std::sync::Arc;
use dotenvy::dotenv;
use tokio::sync::RwLock;
use crate::app::{AppConfig, AppState, NebulaApp};

pub mod web;
pub mod app;
pub mod schema;
pub mod cableway;
pub mod database;
pub mod service;
pub mod data;

pub async fn run_server() {
    dotenv().ok().expect("Couldn't parse .env");
    let config = AppConfig::from_env();
    let db = database::connect(&config).await;

    let state = Arc::new(RwLock::new(
        AppState::new_filling_env_defaults(db)
    ));

    let cableway_client = cableway::connect(&config).await;
    cableway_client.publish("internal.status", "Testing!".into()).await.unwrap();
    cableway_client.flush().await.unwrap();

    let app = NebulaApp {
        config,
        cableway: cableway_client,
        state
    };

    web::serve(app).await
}
