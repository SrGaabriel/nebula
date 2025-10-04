pub mod status;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use crate::app::NebulaApp;
use crate::web::routing::dto::UserDto;
use crate::web::routing::error::{error, ok, NebulaResponse};
use crate::schema::users;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserObject {
    pub user: UserDto
}

pub async fn get_user(
    State(app): State<NebulaApp>,
    Path(user): Path<String>,
) -> NebulaResponse<UserObject> {
    let user_id = user.parse::<u64>();
    if user_id.is_err() {
        return error(StatusCode::BAD_REQUEST, "Invalid user ID");
    }

    let user_id = user_id.unwrap();
    let user = users::Entity::find_by_id(user_id)
        .one(&app.db)
        .await
        .expect("Failed to query user");

    if user.is_none() {
        return error(StatusCode::NOT_FOUND, "User not found");
    }

    let user = user.unwrap();
    let dto = UserDto::from_model(&user);
    ok(UserObject { user: dto })
}