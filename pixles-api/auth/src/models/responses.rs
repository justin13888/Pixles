use super::UserProfile;
use super::errors::*;
use crate::claims::Claims;
use crate::errors::{AuthError, ClaimValidationError};
use aide::OperationOutput;
use aide::openapi::{Operation, Response, StatusCode};
use axum::Json;
use axum::http::StatusCode as HttpStatusCode;
use schemars::JsonSchema;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct TokenResponse {
    #[schemars(with = "String")]
    #[serde(serialize_with = "crate::models::serialize_secret")]
    pub access_token: SecretString,
    #[schemars(with = "String")]
    #[serde(serialize_with = "crate::models::serialize_secret")]
    pub refresh_token: SecretString,
    /// E.g. "Bearer"
    pub token_type: String,
    /// Access token expiry in seconds
    pub expires_by: u64,
}

pub enum RegisterUserResponses {
    Success(TokenResponse),
    BadRequest(BadRegisterUserRequestError),
    UserAlreadyExists,
    InternalServerError(InternalServerError),
}

impl From<Result<TokenResponse, AuthError>> for RegisterUserResponses {
    fn from(result: Result<TokenResponse, AuthError>) -> Self {
        match result {
            Ok(token) => Self::Success(token),
            Err(AuthError::UserAlreadyExists) => Self::UserAlreadyExists,
            Err(AuthError::BadRequest(e)) => Self::BadRequest(e),
            Err(e) => Self::InternalServerError(e.into()),
        }
    }
}

impl axum::response::IntoResponse for RegisterUserResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(token_response) => {
                let status = HttpStatusCode::CREATED;
                let body = Json(token_response);
                (status, body).into_response()
            }
            Self::BadRequest(e) => {
                let status = HttpStatusCode::BAD_REQUEST;
                let body = Json(e);
                (status, body).into_response()
            }
            Self::UserAlreadyExists => {
                let status = HttpStatusCode::CONFLICT;
                let body = Json(ApiError::new("User already exists"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

impl OperationOutput for RegisterUserResponses {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        let mut responses = Vec::new();

        // 201 Created - Success with TokenResponse
        if let Some(resp) =
            <Json<TokenResponse> as OperationOutput>::operation_response(ctx, operation)
        {
            responses.push((Some(201), resp));
        }

        // 400 Bad Request
        responses.push((
            Some(400),
            Response {
                description: "Bad request - invalid registration data".into(),
                ..Default::default()
            },
        ));

        // 409 Conflict
        responses.push((
            Some(409),
            Response {
                description: "User already exists".into(),
                ..Default::default()
            },
        ));

        // 500 Internal Server Error
        responses.push((
            Some(500),
            Response {
                description: "Internal server error".into(),
                ..Default::default()
            },
        ));

        responses
    }
}

pub enum LoginResponses {
    Success(TokenResponse),
    InvalidCredentials,
    InternalServerError(InternalServerError),
}

impl From<Result<TokenResponse, AuthError>> for LoginResponses {
    fn from(result: Result<TokenResponse, AuthError>) -> Self {
        match result {
            Ok(token) => Self::Success(token),
            Err(AuthError::InvalidCredentials) => Self::InvalidCredentials,
            Err(e) => Self::InternalServerError(e.into()),
        }
    }
}

impl axum::response::IntoResponse for LoginResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(token_response) => {
                let status = HttpStatusCode::OK;
                let body = Json(token_response);
                (status, body).into_response()
            }
            Self::InvalidCredentials => {
                let status = HttpStatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new("Invalid credentials"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

impl OperationOutput for LoginResponses {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        let mut responses = Vec::new();

        if let Some(resp) =
            <Json<TokenResponse> as OperationOutput>::operation_response(ctx, operation)
        {
            responses.push((Some(200), resp));
        }

        responses.push((
            Some(401),
            Response {
                description: "Invalid credentials".into(),
                ..Default::default()
            },
        ));

        responses.push((
            Some(500),
            Response {
                description: "Internal server error".into(),
                ..Default::default()
            },
        ));

        responses
    }
}

pub enum RefreshTokenResponses {
    Success(TokenResponse),
    InvalidRefreshToken(AuthError),
    InternalServerError(InternalServerError),
}

impl From<Result<TokenResponse, AuthError>> for RefreshTokenResponses {
    fn from(result: Result<TokenResponse, AuthError>) -> Self {
        match result {
            Ok(token) => Self::Success(token),
            Err(e) => Self::InternalServerError(e.into()),
        }
    }
}

impl axum::response::IntoResponse for RefreshTokenResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(token_response) => {
                let status = HttpStatusCode::OK;
                let body = Json(token_response);
                (status, body).into_response()
            }
            Self::InvalidRefreshToken(e) => {
                let status = HttpStatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new(e.to_string()));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

impl OperationOutput for RefreshTokenResponses {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        let mut responses = Vec::new();

        if let Some(resp) =
            <Json<TokenResponse> as OperationOutput>::operation_response(ctx, operation)
        {
            responses.push((Some(200), resp));
        }

        responses.push((
            Some(401),
            Response {
                description: "Invalid or expired refresh token".into(),
                ..Default::default()
            },
        ));

        responses.push((
            Some(500),
            Response {
                description: "Internal server error".into(),
                ..Default::default()
            },
        ));

        responses
    }
}

pub enum ValidateTokenResponses {
    Valid(String),
    Invalid(AuthError),
}

impl From<Result<Claims, ClaimValidationError>> for ValidateTokenResponses {
    fn from(result: Result<Claims, ClaimValidationError>) -> Self {
        match result {
            Ok(claims) => Self::Valid(claims.sub),
            Err(e) => Self::Invalid(e.into()),
        }
    }
}

impl axum::response::IntoResponse for ValidateTokenResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Valid(token_response) => {
                let status = HttpStatusCode::OK;
                let body = Json(token_response);
                (status, body).into_response()
            }
            Self::Invalid(e) => {
                let status = HttpStatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new(e.to_string()));
                (status, body).into_response()
            }
        }
    }
}

impl OperationOutput for ValidateTokenResponses {
    type Inner = Self;

    fn inferred_responses(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![
            (
                Some(200),
                Response {
                    description: "Token is valid - returns user ID".into(),
                    ..Default::default()
                },
            ),
            (
                Some(401),
                Response {
                    description: "Invalid or expired token".into(),
                    ..Default::default()
                },
            ),
        ]
    }
}

pub enum ResetPasswordRequestResponses {
    Success,
    InternalServerError(InternalServerError),
}

impl axum::response::IntoResponse for ResetPasswordRequestResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success => {
                let status = HttpStatusCode::OK;
                let body = Json(ApiError::new("Password reset request sent"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

impl OperationOutput for ResetPasswordRequestResponses {
    type Inner = Self;

    fn inferred_responses(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![
            (
                Some(200),
                Response {
                    description: "Password reset email sent (if user exists)".into(),
                    ..Default::default()
                },
            ),
            (
                Some(500),
                Response {
                    description: "Internal server error".into(),
                    ..Default::default()
                },
            ),
        ]
    }
}

pub enum PasswordResetResponses {
    Success,
    InvalidToken,
    InvalidNewPassword,
    InternalServerError(InternalServerError),
}

impl axum::response::IntoResponse for PasswordResetResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success => {
                let status = HttpStatusCode::OK;
                let body = Json(ApiError::new("Password reset successful"));
                (status, body).into_response()
            }
            Self::InvalidToken => {
                let status = HttpStatusCode::BAD_REQUEST;
                let body = Json(ApiError::new("Invalid or expired token"));
                (status, body).into_response()
            }
            Self::InvalidNewPassword => {
                let status = HttpStatusCode::BAD_REQUEST;
                let body = Json(ApiError::new("Invalid new password"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

impl OperationOutput for PasswordResetResponses {
    type Inner = Self;

    fn inferred_responses(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![
            (
                Some(200),
                Response {
                    description: "Password reset successful".into(),
                    ..Default::default()
                },
            ),
            (
                Some(400),
                Response {
                    description: "Invalid or expired token, or invalid new password".into(),
                    ..Default::default()
                },
            ),
            (
                Some(500),
                Response {
                    description: "Internal server error".into(),
                    ..Default::default()
                },
            ),
        ]
    }
}

pub enum UserProfileResponses {
    Success(UserProfile),
    Unauthorized(AuthError),
    UserNotFound,
    InternalServerError(InternalServerError),
}

impl axum::response::IntoResponse for UserProfileResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(user_profile) => {
                let status = HttpStatusCode::OK;
                let body = Json(user_profile);
                (status, body).into_response()
            }
            Self::Unauthorized(e) => {
                let status = HttpStatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new(e.to_string()));
                (status, body).into_response()
            }
            Self::UserNotFound => {
                let status = HttpStatusCode::NOT_FOUND;
                let body = Json(ApiError::new("User not found"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

impl OperationOutput for UserProfileResponses {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        let mut responses = Vec::new();

        if let Some(resp) =
            <Json<UserProfile> as OperationOutput>::operation_response(ctx, operation)
        {
            responses.push((Some(200), resp));
        }

        responses.push((
            Some(401),
            Response {
                description: "Unauthorized - invalid or missing token".into(),
                ..Default::default()
            },
        ));

        responses.push((
            Some(404),
            Response {
                description: "User not found".into(),
                ..Default::default()
            },
        ));

        responses.push((
            Some(500),
            Response {
                description: "Internal server error".into(),
                ..Default::default()
            },
        ));

        responses
    }
}

pub enum UpdateUserProfileResponses {
    Success(UserProfile),
    Unauthorized(AuthError),
    InvalidPassword,
    UserNotFound,
    InternalServerError(InternalServerError),
}

impl axum::response::IntoResponse for UpdateUserProfileResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(profile) => {
                let status = HttpStatusCode::OK;
                let body = Json(profile);
                (status, body).into_response()
            }
            Self::Unauthorized(e) => {
                let status = HttpStatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new(e.to_string()));
                (status, body).into_response()
            }
            Self::InvalidPassword => {
                let status = HttpStatusCode::BAD_REQUEST;
                let body = Json(ApiError::new("Invalid password"));
                (status, body).into_response()
            }
            Self::UserNotFound => {
                let status = HttpStatusCode::NOT_FOUND;
                let body = Json(ApiError::new("User not found"));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

impl OperationOutput for UpdateUserProfileResponses {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        let mut responses = Vec::new();

        if let Some(resp) =
            <Json<UserProfile> as OperationOutput>::operation_response(ctx, operation)
        {
            responses.push((Some(200), resp));
        }

        responses.push((
            Some(400),
            Response {
                description: "Invalid password".into(),
                ..Default::default()
            },
        ));

        responses.push((
            Some(401),
            Response {
                description: "Unauthorized - invalid or missing token".into(),
                ..Default::default()
            },
        ));

        responses.push((
            Some(404),
            Response {
                description: "User not found".into(),
                ..Default::default()
            },
        ));

        responses.push((
            Some(500),
            Response {
                description: "Internal server error".into(),
                ..Default::default()
            },
        ));

        responses
    }
}

pub enum LogoutResponses {
    Success,
    Unauthorized(AuthError),
    InternalServerError(InternalServerError),
}

impl axum::response::IntoResponse for LogoutResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success => {
                let status = HttpStatusCode::OK;
                let body = Json(ApiError::new("Logout successful"));
                (status, body).into_response()
            }
            Self::Unauthorized(e) => {
                let status = HttpStatusCode::UNAUTHORIZED;
                let body = Json(ApiError::new(e.to_string()));
                (status, body).into_response()
            }
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

impl OperationOutput for LogoutResponses {
    type Inner = Self;

    fn inferred_responses(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![
            (
                Some(200),
                Response {
                    description: "Logout successful".into(),
                    ..Default::default()
                },
            ),
            (
                Some(401),
                Response {
                    description: "Unauthorized - invalid or missing token".into(),
                    ..Default::default()
                },
            ),
            (
                Some(500),
                Response {
                    description: "Internal server error".into(),
                    ..Default::default()
                },
            ),
        ]
    }
}
