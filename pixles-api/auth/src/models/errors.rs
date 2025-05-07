use argon2::password_hash;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::{ToResponse, ToSchema};

#[derive(Error, Debug)]
pub enum RegisterUserError {
    #[error("Invalid username")]
    InvalidUsername,
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Username already exists")]
    UserAlreadyExists,
} // TODO: Ensure these errors aren't too descriptive for security reasons

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Invalid credentials")]
    InvalidCredentials,
}

#[derive(ToSchema, ToResponse)]
#[schema(description = "Internal server error")]
#[response(description = "Internal server error")]
pub struct InternalServerError {
    pub error: String,
}
// TODO: Somehow automatically track errors ^^

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

impl IntoResponse for InternalServerError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            if cfg!(debug_assertions)
            {
                self.error
            }
            else
            {
                "Unknown internal server error".to_string()
            },
        )
            .into_response()
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
