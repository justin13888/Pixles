use argon2::password_hash;
use salvo::http::StatusCode;
use salvo::oapi::ToSchema;
use salvo::prelude::*;
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
#[derive(ToSchema, Serialize, Deserialize)]
#[salvo(schema(description = "Internal server error"))]
pub struct InternalServerErrorSchema {
    pub _error: String,
}

#[async_trait]
impl Writer for InternalServerError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        tracing::error!(?self, "Internal server error");
        let response = if cfg!(debug_assertions) {
            self.to_string()
        } else {
            "Unknown internal server error".to_string()
        };
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Text::Plain(response));
    }
}

/// A generic API error response
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    pub error: String,
}

impl ApiError {
    pub fn new(error: impl Into<String>) -> Self {
        let error = error.into();
        ApiError { error }
    }
}

#[derive(Serialize, Deserialize, Error, ToSchema, Debug)]
pub enum BadRegisterUserRequestError {
    #[error("Invalid email")]
    Email,
    #[error("Invalid username")]
    Username,
    #[error("Invalid password")]
    Password,
    #[error("Invalid request")]
    InvalidRequest,
}
