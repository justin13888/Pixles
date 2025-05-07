use axum::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::errors::*;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    /// E.g. "Bearer"
    pub token_type: String,
    /// Access token expiry in seconds
    pub expires_by: u64,
}

#[derive(utoipa::IntoResponses)]
pub enum RegisterUserResponses {
    #[response(status = 201, description = "User successfully registered")]
    Success(TokenResponse),
    #[response(status = 400, description = "Invalid username")]
    InvalidUsername,
    #[response(status = 400, description = "Invalid email")]
    InvalidEmail,
    #[response(status = 400, description = "Invalid password")]
    InvalidPassword,
    #[response(status = 409, description = "User already exists")]
    UserAlreadyExists,
    #[response(status = 500, description = "Internal server error")]
    InternalServerError(#[ref_response] InternalServerError),
}

impl axum::response::IntoResponse for RegisterUserResponses {
    fn into_response(self) -> axum::response::Response {
        match (self)
        {
            Self::Success(token_response) =>
            {
                let status = StatusCode::CREATED;
                let body = Json(token_response);
                (status, body).into_response()
            }
            Self::InvalidUsername =>
            {
                let status = StatusCode::BAD_REQUEST;
                let body = Json(ApiError::new("Invalid username"));
                (status, body).into_response()
            }
            Self::InvalidEmail =>
            {
                let status = StatusCode::BAD_REQUEST;
                let body = Json(ApiError::new("Invalid email"));
                (status, body).into_response()
            }
            Self::InvalidPassword =>
            {
                let status = StatusCode::BAD_REQUEST;
                let body = Json(ApiError::new("Invalid password"));
                (status, body).into_response()
            }
            Self::UserAlreadyExists =>
            {
                let status = StatusCode::CONFLICT;
                let body = Json(ApiError::new("User already exists"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) =>
            {
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                let body = Json(ApiError::new("Internal server error"));
                (status, body).into_response()
            }
        }
    }
}

#[derive(utoipa::IntoResponses)]
pub enum LoginResponses {
    /// Success response
    #[response(status = 200, description = "Login successful")]
    Success(TokenResponse),

    #[response(status = 404, description = "Invalid credentials")]
    InvalidCredentials,

    #[response(status = 500, description = "Internal server error")]
    InternalServerError(#[ref_response] InternalServerError),
}

impl axum::response::IntoResponse for LoginResponses {
    fn into_response(self) -> axum::response::Response {
        match self
        {
            Self::Success(token_response) =>
            {
                let status = StatusCode::OK;
                let body = Json(token_response);
                (status, body).into_response()
            }
            Self::InvalidCredentials =>
            {
                let status = StatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new("Invalid credentials"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) =>
            {
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                let body = Json(ApiError::new("Internal server error"));
                (status, body).into_response()
            }
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum ValidateTokenResponse {
    Valid(String),
    Invalid,
}
