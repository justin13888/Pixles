use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User not found or invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken(#[from] ClaimValidationError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidCredentials => (StatusCode::NOT_FOUND, "User not found".to_string()),
            AuthError::InvalidToken(e) => (StatusCode::UNAUTHORIZED, e.to_string()),
        };

        (status, error_message).into_response()
    }
}

// JWT validation error
#[derive(Error, Debug)]
pub enum ClaimValidationError {
    #[error("Missing token")]
    TokenMissing,
    #[error("Invalid token: {0}")]
    TokenInvalid(#[from] jsonwebtoken::errors::Error),
    #[error("Expired token")]
    TokenExpired,
    #[error("Unexpected authorization header format")]
    UnexpectedHeaderFormat,
    #[error("Invalid scopes")]
    InvalidScopes,
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

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        AuthError::InvalidToken(ClaimValidationError::TokenInvalid(error))
    }
}

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

// TODO: is this necessary vv
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    pub message: String,
    pub status_code: u16,
}
