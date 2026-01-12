use salvo::http::StatusCode;
use salvo::prelude::*;
use thiserror::Error;

use crate::models::errors::BadRegisterUserRequestError;
use model::errors::InternalServerError;

/// Register user error
#[derive(Debug)]
pub enum RegisterError {
    UserAlreadyExists,
    BadRequest(BadRegisterUserRequestError),
    Unexpected(InternalServerError),
}

impl std::fmt::Display for RegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserAlreadyExists => write!(f, "User already exists"),
            Self::BadRequest(e) => write!(f, "Bad request: {}", e),
            Self::Unexpected(e) => write!(f, "Internal server error: {}", e),
        }
    }
}

impl From<BadRegisterUserRequestError> for RegisterError {
    fn from(err: BadRegisterUserRequestError) -> Self {
        Self::BadRequest(err)
    }
}

impl From<InternalServerError> for RegisterError {
    fn from(err: InternalServerError) -> Self {
        Self::Unexpected(err)
    }
}

#[async_trait]
impl Writer for RegisterError {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            RegisterError::UserAlreadyExists => {
                res.status_code(StatusCode::CONFLICT);
                res.render(Text::Plain("User already exists"));
            }
            RegisterError::BadRequest(e) => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Text::Plain(e.to_string()));
            }
            RegisterError::Unexpected(e) => {
                e.write(req, depot, res).await;
            }
        }
    }
}

/// Login error
#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials,
    AccountNotVerified,
    Unexpected(InternalServerError),
}

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCredentials => write!(f, "User not found or invalid credentials"),
            Self::AccountNotVerified => write!(f, "Account not verified"),
            Self::Unexpected(e) => write!(f, "Internal server error: {}", e),
        }
    }
}

impl From<InternalServerError> for LoginError {
    fn from(err: InternalServerError) -> Self {
        Self::Unexpected(err)
    }
}

impl From<sea_orm::DbErr> for LoginError {
    fn from(err: sea_orm::DbErr) -> Self {
        InternalServerError::from(err).into()
    }
}

#[async_trait]
impl Writer for LoginError {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            LoginError::InvalidCredentials => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Text::Plain("Invalid credentials"));
            }
            LoginError::AccountNotVerified => {
                res.status_code(StatusCode::FORBIDDEN);
                res.render(Text::Plain("Account not verified"));
            }
            LoginError::Unexpected(e) => {
                e.write(req, depot, res).await;
            }
        }
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

/// TOTP enrollment error
#[derive(Debug, Error)]
pub enum TotpEnrollError {
    #[error("User not found")]
    UserNotFound,
    #[error("TOTP is already enabled")]
    AlreadyEnabled,
    #[error("Database error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] eyre::Report),
}

/// TOTP verification error
#[derive(Debug, Error)]
pub enum TotpVerificationError {
    #[error("User not found")]
    UserNotFound,
    #[error("TOTP is not enabled")]
    NotEnabled,
    #[error("Invalid code")]
    InvalidCode,
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] eyre::Report),
}

/// Passkey registration error
#[derive(Debug, Error)]
pub enum PasskeyRegistrationError {
    #[error("User not found")]
    UserNotFound,
    #[error("Passkey already exists")]
    AlreadyExists,
    #[error("Registration failed: {0}")]
    RegistrationFailed(String),
    #[error("Invalid challenge")]
    InvalidChallenge,
    #[error("Limit reached: {0}")]
    LimitReached(String),
    #[error("Database error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] eyre::Report),
}

/// Passkey authentication error
#[derive(Debug, Error)]
pub enum PasskeyAuthenticationError {
    #[error("User not found")]
    UserNotFound, // Used for start_authentication
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String), // e.g. user verification failed
    #[error("Invalid credential")]
    InvalidCredential, // Used for finish_authentication
    #[error("Database error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] eyre::Report),
}

/// Passkey management error
#[derive(Debug, Error)]
pub enum PasskeyManagementError {
    #[error("User not found")]
    UserNotFound,
    #[error("Passkey not found")]
    NotFound,
    #[error("Database error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] eyre::Report),
}
