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
pub mod util;

#[tokio::main]
pub async fn main() {
    dotenv().ok().expect("Couldn't parse .env");
    let config = AppConfig::from_env();
    let db = database::connect(&config).await;

    let state = Arc::new(RwLock::new(
        AppState::new()
    ));

    let cableway_client = cableway::connect(&config).await;
    let app = NebulaApp {
        config,
        cableway: cableway_client,
        state,
        db
    };

    web::serve(app).await
}

