//! Public error types for auth library

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use utoipa::{PartialSchema, ToResponse, ToSchema};

/// Authentication error
#[derive(Error, Debug, ToSchema)]
pub enum AuthError {
    #[error("User not found or invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken(#[from] ClaimValidationError),
    #[error("Internal server error")]
    #[schema(value_type = String)]
    InternalServerError(#[from] eyre::Report), // Using eyre::Report or Box<dyn StdError> or similar
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidCredentials => (StatusCode::NOT_FOUND, "User not found".to_string()),
            AuthError::InvalidToken(e) => (StatusCode::UNAUTHORIZED, e.to_string()),
            AuthError::InternalServerError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ), // Don't expose internal error details
        };

        (status, error_message).into_response()
    }
}

// JWT validation error
#[derive(Error, Debug, ToSchema)]
pub enum ClaimValidationError {
    #[error("Missing token")]
    TokenMissing,
    #[error("Expired token")]
    TokenExpired,
    #[error("Invalid token: {0}")]
    TokenInvalid(String),
    #[error("Unexpected authorization header format")]
    UnexpectedHeaderFormat,
    #[error("Invalid scopes")]
    InvalidScopes,
}

impl From<jsonwebtoken::errors::Error> for ClaimValidationError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                ClaimValidationError::TokenInvalid(error.to_string())
            }
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => ClaimValidationError::TokenExpired,
            _ => ClaimValidationError::TokenInvalid(error.to_string()),
        }
    }
}

impl IntoResponse for ClaimValidationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ClaimValidationError::TokenMissing => (StatusCode::UNAUTHORIZED, "Token missing"),
            ClaimValidationError::TokenInvalid(_) => (StatusCode::UNAUTHORIZED, "Invalid token"),
            ClaimValidationError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
            ClaimValidationError::UnexpectedHeaderFormat => (
                StatusCode::UNAUTHORIZED,
                "Unexpected authorization header format",
            ),
            ClaimValidationError::InvalidScopes => (StatusCode::FORBIDDEN, "Invalid scopes"),
        };

        (status, error_message).into_response()
    }
}
