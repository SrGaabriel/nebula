use std::collections::BTreeMap;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use jwt::VerifyWithKey;
use crate::app::SharedState;
use crate::web::routing::error::error;
use crate::schema::users;
use sea_orm::EntityTrait;

pub async fn authorize(mut req: Request, next: Next) -> Response {
    let headers = req.headers().clone();
    let auth = headers.get("Authorization").or(headers.get("Sec-Websocket-Protocol"));
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

    let extensions = req.extensions_mut();
    let user = {
        let state = extensions.get::<SharedState>().unwrap().read().await;
        let claims: Result<BTreeMap<String, u64>, jwt::error::Error> = token.unwrap().verify_with_key(&state.jwt_key);
        if claims.is_err() {
            return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
        }
        let claims = claims.unwrap();

        let user_id: Option<&u64> = claims.get("id");
        if user_id.is_none() {
            return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
        }

        users::Entity::find_by_id(*user_id.unwrap())
            .one(&state.db)
            .await
    };
    if user.is_err() {
        return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    }
    
    extensions.insert(user.unwrap());
    next.run(req).await
}