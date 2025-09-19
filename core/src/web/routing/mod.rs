use crate::app::NebulaApp;
use axum::routing::{get, post};
use axum::{middleware, Router};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

mod auth;
mod middlewares;
mod error;
mod users;
mod dto;
mod realms;

pub fn router(app: NebulaApp) -> Router {
    Router::new()
        .route("/api/users/{user}", get(users::get_user))
        .route("/api/realms/{realm_id}", get(realms::get_realm))
        .route_layer(middleware::from_fn_with_state(app.clone(), middlewares::auth::authorize))
        .route("/api/login", post(auth::login::login_handler))
        .route("/api/signup", post(auth::signup::signup_handler))
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true))
                )
                .into_inner()
        )
        .with_state(app)
}