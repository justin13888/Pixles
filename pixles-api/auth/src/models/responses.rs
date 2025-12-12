use axum::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

use super::UserProfile;
use super::errors::*;
use crate::errors::AuthError;

#[derive(Serialize, Deserialize, ToSchema, ToResponse)]
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
    #[response(status = 400, description = "Bad request")]
    BadRequest(BadRegisterUserRequestError),
    #[response(status = 409, description = "User already exists")]
    UserAlreadyExists,
    #[response(status = 500, description = "Internal server error")]
    InternalServerError(#[ref_response] InternalServerError),
}

impl axum::response::IntoResponse for RegisterUserResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(token_response) => {
                let status = StatusCode::CREATED;
                let body = Json(token_response);
                (status, body).into_response()
            }
            Self::BadRequest(e) => {
                let status = StatusCode::BAD_REQUEST;
                let body = Json(e);
                (status, body).into_response()
            }
            Self::UserAlreadyExists => {
                let status = StatusCode::CONFLICT;
                let body = Json(ApiError::new("User already exists"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

#[derive(utoipa::IntoResponses)]
pub enum LoginResponses {
    #[response(status = 200, description = "Login successful")]
    Success(TokenResponse),

    #[response(status = 404, description = "Invalid credentials")]
    InvalidCredentials,

    #[response(status = 500, description = "Internal server error")]
    InternalServerError(#[ref_response] InternalServerError),
}

impl axum::response::IntoResponse for LoginResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(token_response) => {
                let status = StatusCode::OK;
                let body = Json(token_response);
                (status, body).into_response()
            }
            Self::InvalidCredentials => {
                let status = StatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new("Invalid credentials"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, ToResponse)]
pub enum ValidateTokenResponse {
    Valid(String),
    Invalid,
}

#[derive(utoipa::IntoResponses)]
pub enum RefreshTokenResponses {
    #[response(status = 200, description = "Token refreshed successfully")]
    Success(TokenResponse),

    #[response(status = 401, description = "Invalid refresh token")]
    InvalidRefreshToken(AuthError),

    #[response(status = 500, description = "Internal server error")]
    InternalServerError(#[ref_response] InternalServerError),
}

impl axum::response::IntoResponse for RefreshTokenResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(token_response) => {
                let status = StatusCode::OK;
                let body = Json(token_response);
                (status, body).into_response()
            }
            Self::InvalidRefreshToken(e) => {
                let status = StatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new(e.to_string()));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

#[derive(utoipa::IntoResponses)]
pub enum ValidateTokenResponses {
    #[response(status = 200, description = "Token is valid")]
    Valid(ValidateTokenResponse),

    #[response(status = 401, description = "Invalid token")]
    Invalid(AuthError),
}

impl axum::response::IntoResponse for ValidateTokenResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Valid(token_response) => {
                let status = StatusCode::OK;
                let body = Json(token_response);
                (status, body).into_response()
            }
            Self::Invalid(e) => {
                let status = StatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new(e.to_string()));
                (status, body).into_response()
            }
        }
    }
}

#[derive(utoipa::IntoResponses)]
pub enum ResetPasswordRequestResponses {
    /// Success response
    #[response(status = 200, description = "Password reset request sent if it exists")]
    Success,

    #[response(status = 500, description = "Internal server error")]
    InternalServerError(#[ref_response] InternalServerError),
}

impl axum::response::IntoResponse for ResetPasswordRequestResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success => {
                let status = StatusCode::OK;
                let body = Json(ApiError::new("Password reset request sent"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

#[derive(utoipa::IntoResponses)]
pub enum PasswordResetResponses {
    #[response(status = 200, description = "Password reset successful")]
    Success,

    #[response(status = 400, description = "Invalid or expired token")]
    InvalidToken,

    #[response(status = 400, description = "Invalid new password")]
    InvalidNewPassword,

    #[response(status = 500, description = "Internal server error")]
    InternalServerError(#[ref_response] InternalServerError),
}

impl axum::response::IntoResponse for PasswordResetResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success => {
                let status = StatusCode::OK;
                let body = Json(ApiError::new("Password reset successful"));
                (status, body).into_response()
            }
            Self::InvalidToken => {
                let status = StatusCode::BAD_REQUEST;
                let body = Json(ApiError::new("Invalid or expired token"));
                (status, body).into_response()
            }
            Self::InvalidNewPassword => {
                let status = StatusCode::BAD_REQUEST;
                let body = Json(ApiError::new("Invalid new password"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

#[derive(utoipa::IntoResponses)]
pub enum UserProfileResponses {
    #[response(status = 200, description = "User profile retrieved successfully")]
    Success(UserProfile),

    #[response(status = 401, description = "Unauthorized")]
    Unauthorized(AuthError),

    #[response(status = 404, description = "User not found")]
    UserNotFound,

    #[response(status = 500, description = "Internal server error")]
    InternalServerError(#[ref_response] InternalServerError),
}

impl axum::response::IntoResponse for UserProfileResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(user_profile) => {
                let status = StatusCode::OK;
                let body = Json(user_profile);
                (status, body).into_response()
            }
            Self::Unauthorized(e) => {
                let status = StatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new(e.to_string()));
                (status, body).into_response()
            }
            Self::UserNotFound => {
                let status = StatusCode::NOT_FOUND;
                let body = Json(ApiError::new("User not found"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

#[derive(utoipa::IntoResponses)]
pub enum UpdateUserProfileResponses {
    #[response(status = 200, description = "User profile updated successfully")]
    Success(UserProfile),
    // #[response(status = 400, description = "Invalid user profile data")]
    // InvalidData,
    #[response(status = 401, description = "Unauthorized")]
    Unauthorized(AuthError),

    #[response(status = 400, description = "Invalid password")]
    InvalidPassword,

    #[response(status = 404, description = "User not found")]
    UserNotFound,

    #[response(status = 500, description = "Internal server error")]
    InternalServerError(#[ref_response] InternalServerError),
}

impl axum::response::IntoResponse for UpdateUserProfileResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(profile) => {
                let status = StatusCode::OK;
                let body = Json(profile);
                (status, body).into_response()
            }
            Self::Unauthorized(e) => {
                let status = StatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new(e.to_string()));
                (status, body).into_response()
            }
            Self::InvalidPassword => {
                let status = StatusCode::BAD_REQUEST;
                let body = Json(ApiError::new("Invalid password"));
                (status, body).into_response()
            }
            Self::UserNotFound => {
                let status = StatusCode::NOT_FOUND;
                let body = Json(ApiError::new("User not found"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

#[derive(utoipa::IntoResponses)]
pub enum LogoutResponses {
    #[response(status = 200, description = "Logout successful")]
    Success,

    #[response(status = 401, description = "Unauthorized")]
    Unauthorized(AuthError),

    #[response(status = 500, description = "Internal server error")]
    InternalServerError(#[ref_response] InternalServerError),
}

impl axum::response::IntoResponse for LogoutResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success => {
                let status = StatusCode::OK;
                let body = Json(ApiError::new("Logout successful"));
                (status, body).into_response()
            }
            Self::Unauthorized(e) => {
                let status = StatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new(e.to_string()));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}
