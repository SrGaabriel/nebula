use crate::app::NebulaApp;
use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use crate::realm_membership;

pub mod auth;
pub mod middlewares;
pub mod error;
pub mod users;
pub mod dto;
pub mod realms;

pub fn router(app: NebulaApp) -> Router {
    Router::new()
        .route("/api/users/{user}", get(users::get_user))
        .route("/api/realms/{realm_id}",
               get(realms::get_realm)
                   .layer(realm_membership!(app))
        )
        .route("/api/realms/{realm_id}/calendar/events",
               post(realms::calendar::events::create_event)
                   .layer(realm_membership!(app, [ManageEvents]))
        )
        .route("/api/realms/{realm_id}/calendar/events/{event_id}",
               delete(realms::calendar::events::delete_event)
                   .layer(realm_membership!(app, [ManageEvents]))
        )
        .route("/api/realms/{realm_id}/calendar/schedule",
               get(realms::calendar::occurrences::get_occurrences)
                   .layer(realm_membership!(app))
        )
        .route("/api/realms/{realm_id}/tasks",
               post(realms::task::create_task)
                   .layer(realm_membership!(app, [ManageTasks]))
        )
        .route("/api/realms", post(realms::create::create_realm))
        .route_layer(middleware::from_fn_with_state(app.clone(), middlewares::auth::authorize))
        .route("/api/login", post(auth::login::login_handler))
        .route("/api/signup", post(auth::signup::signup_handler))
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true))
                )
        )
        .with_state(app)
}