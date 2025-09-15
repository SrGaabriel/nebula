use std::collections::BTreeMap;

use axum::{extract::State, http::StatusCode, Json};
use jwt::SignWithKey;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use argon2::PasswordHasher;

use crate::{app::NebulaApp, schema::users, service::snowflake::next_snowflake, web::routing::error::{error, ok, NebulaResponse}};

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SignupResponse {
    pub user_id: u64,
    pub token: String,
}

pub async fn signup_handler(
    State(app): State<NebulaApp>,
    Json(payload): Json<SignupRequest>
) -> NebulaResponse<SignupResponse> {
    let state = &app.state.read().await;
    let user_with_same_email = users::Entity::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(&state.db)
        .await
        .expect("Failed to query the database");

    if user_with_same_email.is_some() {
        return error(
            StatusCode::CONFLICT,
            "A user with the same email already exists"
        );
    }

    let password_hash = &state.argon.hash_password(
        payload.password.as_bytes(),
        &app.config.argon_salt
    ).expect("Failed to hash password").to_string();

    let user_id = next_snowflake();
    let user = users::ActiveModel {
        id: Set(user_id as i64),
        name: Set(payload.name),
        email: Set(payload.email),
        password_hash: Set(password_hash.to_owned()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
    };
    user.insert(&state.db)
        .await
        .expect("Failed to insert new user");

    let mut token_claims = BTreeMap::new();
    token_claims.insert("user_id", user_id.to_string());
    let token = token_claims.sign_with_key(&state.jwt_key)
        .expect("Failed to sign JWT");

    ok(SignupResponse {
        user_id,
        token,
    })
}