use argon2::password_hash;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Error, ToResponse)]
#[response(description = "Internal server error")]
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
#[derive(ToSchema, ToResponse)]
#[schema(as = InternalServerError, description = "Internal server error")]
#[response(description = "Internal server error")]
pub struct InternalServerErrorSchema {
    pub _error: String,
}

impl utoipa::PartialSchema for InternalServerError {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        InternalServerErrorSchema::schema()
    }
}

impl ToSchema for InternalServerError {
    fn schemas(
        schemas: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        InternalServerErrorSchema::schemas(schemas)
    }
}

// TODO: Somehow automatically track errors in logs ^^

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
