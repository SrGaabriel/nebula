pub mod web;
pub mod app;
pub mod schema;
pub mod cableway;
pub mod database;
pub mod service;
pub mod data;

use std::sync::Arc;
use dotenvy::dotenv;
use tokio::sync::RwLock;
use nebula_core::run_server;
use crate::app::{AppConfig, AppState, NebulaApp};

#[tokio::main]
pub async fn main() {
    run_server().await
}
