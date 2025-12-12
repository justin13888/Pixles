use argon2::password_hash;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InternalServerError {
    #[error("SeaORM error: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Password hash error: {0}")]
    PasswordHash(password_hash::errors::Error),

    #[error("Auth error: {0}")]
    Auth(#[from] crate::errors::AuthError),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Internal error: {0}")]
    Eyre(#[from] eyre::Report),
}

impl From<password_hash::errors::Error> for InternalServerError {
    fn from(error: password_hash::errors::Error) -> Self {
        Self::PasswordHash(error)
    }
}

/// Proxy struct to generate the correct API schema for InternalServerError
#[derive(JsonSchema, Serialize, Deserialize)]
#[schemars(description = "Internal server error")]
pub struct InternalServerErrorSchema {
    pub _error: String,
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> Response {
        tracing::error!(?self, "Internal server error");
        let response = if cfg!(debug_assertions) {
            self.to_string()
        } else {
            "Unknown internal server error".to_string()
        };
        (StatusCode::INTERNAL_SERVER_ERROR, response).into_response()
    }
}

/// A generic API error response
#[derive(Serialize, Deserialize, JsonSchema)]
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

#[derive(Serialize, Deserialize, Error, JsonSchema, Debug)]
pub enum BadRegisterUserRequestError {
    #[error("Invalid email")]
    Email,
    #[error("Invalid username")]
    Username,
    #[error("Invalid password")]
    Password,
}
