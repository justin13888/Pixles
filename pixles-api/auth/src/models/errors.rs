use argon2::password_hash;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::{ToResponse, ToSchema};

// TODO: Convert this to use thiserror::Error trait

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
            error: format!("SeaORM error: {error}"),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for InternalServerError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        InternalServerError {
            error: format!("JWT error: {error}"),
        }
    }
}

impl From<password_hash::errors::Error> for InternalServerError {
    fn from(error: password_hash::errors::Error) -> Self {
        InternalServerError {
            error: format!("Password hash error: {error}"),
        }
    }
}

impl From<crate::errors::AuthError> for InternalServerError {
    fn from(error: crate::errors::AuthError) -> Self {
        InternalServerError {
            error: format!("Auth error: {error}"),
        }
    }
}

impl From<redis::RedisError> for InternalServerError {
    fn from(error: redis::RedisError) -> Self {
        InternalServerError {
            error: format!("Redis error: {error}"),
        }
    }
}

impl From<eyre::Report> for InternalServerError {
    fn from(error: eyre::Report) -> Self {
        InternalServerError {
            error: format!("Internal error: {error}"),
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
        let response = if cfg!(debug_assertions) {
            self.to_string()
        } else {
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

#[derive(Serialize, Deserialize, Error, ToSchema, Debug)]
pub enum BadRegisterUserRequestError {
    #[schema(rename = "Invalid email")]
    #[error("Invalid email")]
    Email,
    #[schema(rename = "Invalid username")]
    #[error("Invalid username")]
    Username,
    #[schema(rename = "Invalid password")]
    #[error("Invalid password")]
    Password,
}
