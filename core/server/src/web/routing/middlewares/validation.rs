use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{FromRequestParts, Request};
use axum::Json;
use axum::response::IntoResponse;
use garde::{Report, Validate};
use serde::de::DeserializeOwned;

pub struct ValidJson<T : DeserializeOwned + Validate>(pub T);

impl <T: DeserializeOwned + Validate, S : Send + Sync> axum::extract::FromRequest<S> for ValidJson<T>
    where <T as Validate>::Context : Default
{
    type Rejection = ValidJsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(ValidJsonRejection::InvalidJson)?;
        match value.validate() {
            Ok(_) => {},
            Err(err) => return Err(ValidJsonRejection::InvalidData(err))
        }
        Ok(ValidJson(value))
    }
}

pub enum ValidJsonRejection {
    InvalidJson(JsonRejection),
    InvalidData(Report)
}

impl IntoResponse for ValidJsonRejection {
    #[inline]
    fn into_response(self) -> axum::response::Response {
        match self {
            ValidJsonRejection::InvalidJson(rejection) => rejection.into_response(),
            ValidJsonRejection::InvalidData(report) => {
                (axum::http::StatusCode::BAD_REQUEST, Json(report)).into_response()
            }
        }
    }
}

pub struct ValidQuery<T : DeserializeOwned + Validate>(pub T);

impl <T: DeserializeOwned + Validate, S: Send + Sync> FromRequestParts<S> for ValidQuery<T>
    where <T as Validate>::Context : Default
{
    type Rejection = ValidQueryRejection;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query = axum::extract::Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(ValidQueryRejection::InvalidFormat)?;
        match query.0.validate() {
            Ok(_) => {},
            Err(err) => return Err(ValidQueryRejection::InvalidData(err))
        }
        Ok(ValidQuery(query.0))
    }
}

pub enum ValidQueryRejection {
    InvalidFormat(QueryRejection),
    InvalidData(Report)
}

impl IntoResponse for ValidQueryRejection {
    #[inline]
    fn into_response(self) -> axum::response::Response {
        match self {
            ValidQueryRejection::InvalidFormat(rejection) => rejection.into_response(),
            ValidQueryRejection::InvalidData(report) => {
                (axum::http::StatusCode::BAD_REQUEST, Json(report)).into_response()
            }
        }
    }
}