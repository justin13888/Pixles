use argon2::password_hash;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(ToSchema, ToResponse)]
#[schema(description = "Internal server error")]
#[response(description = "Internal server error")]
pub struct InternalServerError {
    pub error: String,
}
// TODO: Somehow automatically track errors in logs ^^

impl From<sea_orm::DbErr> for InternalServerError {
    fn from(error: sea_orm::DbErr) -> Self {
        InternalServerError {
            error: error.to_string(),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for InternalServerError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        InternalServerError {
            error: error.to_string(),
        }
    }
}

impl From<password_hash::errors::Error> for InternalServerError {
    fn from(error: password_hash::errors::Error) -> Self {
        InternalServerError {
            error: error.to_string(),
        }
    }
}

impl std::fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> Response {
        let response = if cfg!(debug_assertions)
        {
            self.to_string()
        }
        else
        {
            "Unknown internal server error".to_string()
        };
        (StatusCode::INTERNAL_SERVER_ERROR, response).into_response()
    }
}

/// A generic API error response
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    pub error: String,
}
// TODO: Remove this ^^

impl ApiError {
    pub fn new(error: impl Into<String>) -> Self {
        let error = error.into();
        ApiError { error }
    }
}
