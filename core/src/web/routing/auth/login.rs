use super::{MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH};
use crate::app::NebulaApp;
use crate::schema::users;
use crate::web::routing::auth::{generate_jwt_token, AuthResponse};
use crate::web::routing::error::NebulaResponse;
use crate::web::routing::error::{error, ok};
use crate::web::routing::middlewares::validation::ValidJson;
use argon2::{PasswordHash, PasswordVerifier};
use axum::extract::State;
use axum::http::StatusCode;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, garde::Validate)]
pub struct LoginRequest {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = MIN_PASSWORD_LENGTH, max = MAX_PASSWORD_LENGTH))]
    pub password: String,
}

pub async fn login_handler(
    State(app): State<NebulaApp>,
    ValidJson(payload): ValidJson<LoginRequest>
) -> NebulaResponse<AuthResponse> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(&app.db)
        .await
        .expect("Failed to query the database");

    if user.is_none() {
        return error(StatusCode::UNAUTHORIZED, "Invalid email or password");
    }
    let user = user.unwrap();
    let password_hash = PasswordHash::new(&user.password_hash).expect("Failed to hash password");

    let is_password_valid = app.config.argon2.verify_password(
        payload.password.as_bytes(),
        &password_hash
    ).is_ok();
    if !is_password_valid {
        return error(StatusCode::UNAUTHORIZED, "Invalid email or password");
    }
    let dto =  crate::web::routing::dto::UserDto::from_model(&user);

    let token = generate_jwt_token(&app.config.jwt_key, user.id.0);
    ok(AuthResponse {
        user: dto,
        token,
    })
}