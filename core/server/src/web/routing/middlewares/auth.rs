use crate::app::NebulaApp;
use crate::service;
use crate::web::routing::error::error;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

pub async fn authorize(
    State(app): State<NebulaApp>,
    mut req: Request,
    next: Next
) -> Response {
    let headers = req.headers().clone();
    let auth = headers.get("Authorization");
    if auth.is_none() {
        return error::<String>(StatusCode::UNAUTHORIZED, "No authorization header provided").into_response();
    }

    let auth = auth.unwrap().to_str();
    if auth.is_err() {
        return error::<String>(StatusCode::UNAUTHORIZED, "Invalid authorization header").into_response();
    }

    let parts: Vec<&str> = auth.unwrap().split_whitespace().collect();
    let token = parts.last();
    if token.is_none() {
        return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    }

    let user_result = service::auth::authenticate(
        &app.config,
        &app.db,
        token.unwrap().to_string()
    ).await;
    match user_result {
        Ok(user) => {
            req.extensions_mut().insert(user);
            next.run(req).await
        },
        Err(_) => error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response()
    }
}