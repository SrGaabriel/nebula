use axum::http::StatusCode;
use axum::Json;
use axum_extra::either::Either;
use serde::Serialize;

pub type NebulaResponse<T> = (StatusCode, Either<Json<T>, Json<NebulaError>>);

#[derive(Serialize)]
pub struct NebulaError {
    pub status: u16,
    pub message: String
}

pub fn ok<T: Serialize>(data: T) -> NebulaResponse<T> {
    (StatusCode::OK, Either::E1(Json(data)))
}

pub fn no_content() -> NebulaResponse<()> {
    (StatusCode::NO_CONTENT, Either::E1(Json(())))
}

pub fn error<T: Serialize>(status: StatusCode, message: &str) -> NebulaResponse<T> {
    (status, Either::E2(Json(NebulaError {
        status: status.as_u16(),
        message: String::from(message)
    })))
}