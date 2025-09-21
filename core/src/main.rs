pub mod web;
pub mod app;
pub mod schema;
pub mod cableway;
pub mod database;
pub mod service;
pub mod data;

use nebula_core::run_server;

#[tokio::main]
pub async fn main() {
    run_server().await
}
