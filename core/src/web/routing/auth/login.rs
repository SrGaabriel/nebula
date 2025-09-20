use argon2::{PasswordHash, PasswordVerifier};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use crate::app::NebulaApp;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::EntityTrait;
use crate::schema::users;
use crate::web::routing::auth::{generate_jwt_token, AuthResponse};
use crate::web::routing::error::{error, ok};
use crate::web::routing::error::NebulaResponse;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_handler(
    State(app): State<NebulaApp>,
    Json(payload): Json<LoginRequest>
) -> NebulaResponse<AuthResponse> {
    let state = &app.state.read().await;
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(&state.db)
        .await
        .expect("Failed to query the database");

    if user.is_none() {
        return error(StatusCode::UNAUTHORIZED, "Invalid email or password");
    }
    let user = user.unwrap();
    let password_hash = PasswordHash::new(&user.password_hash).expect("Failed to hash password");

    let is_password_valid = app.state.read().await.argon.verify_password(
        payload.password.as_bytes(),
        &password_hash
    ).is_ok();
    if !is_password_valid {
        return error(StatusCode::UNAUTHORIZED, "Invalid email or password");
    }
    let dto =  crate::web::routing::dto::UserDto::from_model(&user);

    let token = generate_jwt_token(&state.jwt_key, user.id.0);
    ok(AuthResponse {
        user: dto,
        token,
    })
}