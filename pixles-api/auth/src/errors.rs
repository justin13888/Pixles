//! Public error types for auth library

use salvo::http::StatusCode;
use salvo::prelude::*;
use thiserror::Error;

use crate::models::errors::BadRegisterUserRequestError;

/// Authentication error
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User not found or invalid credentials")]
    InvalidCredentials,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Bad request")]
    BadRequest(BadRegisterUserRequestError),
    #[error("Invalid token")]
    InvalidToken(#[from] ClaimValidationError),
    #[error("Internal server error")]
    InternalServerError(#[from] eyre::Report),
}

#[async_trait]
impl Writer for AuthError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let (status, error_message) = match self {
            AuthError::InvalidCredentials => (StatusCode::NOT_FOUND, "User not found".to_string()),
            AuthError::UserAlreadyExists => {
                (StatusCode::CONFLICT, "User already exists".to_string())
            }
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AuthError::BadRequest(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AuthError::InvalidToken(e) => (StatusCode::UNAUTHORIZED, e.to_string()),
            AuthError::InternalServerError(_e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        res.status_code(status);
        res.render(Text::Plain(error_message));
    }
}

// JWT validation error
#[derive(Error, Debug)]
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

#[async_trait]
impl Writer for ClaimValidationError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
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

        res.status_code(status);
        res.render(Text::Plain(error_message));
    }
}
