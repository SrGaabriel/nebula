use argon2::PasswordHasher;
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::web::routing::auth::{generate_jwt_token, AuthResponse};
use crate::web::routing::dto::UserDto;
use crate::{app::NebulaApp, schema::users, service::snowflake::next_snowflake, web::routing::error::{error, ok, NebulaResponse}};

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String
}

pub async fn signup_handler(
    State(app): State<NebulaApp>,
    Json(payload): Json<SignupRequest>
) -> NebulaResponse<AuthResponse> {
    let db = &app.db;
    let user_with_same_email = users::Entity::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(db)
        .await
        .expect("Failed to query the database");

    if user_with_same_email.is_some() {
        return error(
            StatusCode::CONFLICT,
            "A user with the same email already exists"
        );
    }

    let password_hash = &app.config.argon2.hash_password(
        payload.password.as_bytes(),
        &app.config.argon_salt
    ).expect("Failed to hash password").to_string();

    let user_id = next_snowflake();
    let user = users::ActiveModel {
        id: Set(user_id),
        name: Set(payload.name.clone()),
        email: Set(payload.email),
        password_hash: Set(password_hash.to_owned()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
    };
    user.insert(db)
        .await
        .expect("Failed to insert new user");
    let token = generate_jwt_token(&app.config.jwt_key, user_id.0);

    ok(AuthResponse {
        user: UserDto {
            id: user_id,
            name: payload.name
        },
        token,
    })
}