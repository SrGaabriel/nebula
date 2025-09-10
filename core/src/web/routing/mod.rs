use async_nats::Client;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use crate::app::SharedState;

mod auth;
mod error;

pub fn router(state: SharedState, cableway: Client) -> Router {
    Router::new()
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true))
                )
                .layer(AddExtensionLayer::new(state))
                .layer(AddExtensionLayer::new(cableway))
                .into_inner()
        )
}