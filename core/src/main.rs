mod web;
mod app;
mod schema;
mod cableway;

use std::sync::Arc;
use dotenvy::dotenv;
use tokio::sync::RwLock;
use crate::app::{AppConfig, AppState, NebulaApp};

#[tokio::main]
async fn main() {
    dotenv().ok().expect("Couldn't parse .env");
    let config = AppConfig::from_env();
    let state = Arc::new(RwLock::new(
        AppState::from_env()
    ));

    let cableway_client = cableway::connect(config).await;

    let app = NebulaApp {
        config,
        cableway: cableway_client,
        state
    };

    web::serve(&app).await
}

