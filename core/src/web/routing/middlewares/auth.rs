use std::collections::BTreeMap;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use jwt::VerifyWithKey;
use crate::app::{NebulaApp};
use crate::web::routing::error::error;
use crate::schema::users;
use sea_orm::EntityTrait;

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

    let user = {
        let state = app.state.read().await;
        let claims: Result<BTreeMap<String, String>, jwt::error::Error> = token.unwrap().verify_with_key(&state.jwt_key);
        if claims.is_err() {
            return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
        }
        let claims = claims.unwrap();

        let user_id_str: Option<&String> = claims.get("user_id");
        if user_id_str.is_none() {
            return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
        }
        let user_id = user_id_str.unwrap().parse::<u64>();
        if user_id.is_err() {
            return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token claim").into_response();
        }

        users::Entity::find_by_id(user_id.unwrap())
            .one(&state.db)
            .await
    };
    if user.is_err() {
        return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    }
    let user = user.unwrap();
    if user.is_none() {
        return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    }

    req.extensions_mut().insert(user.unwrap());
    next.run(req).await
}