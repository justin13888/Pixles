use super::UserProfile;
use super::errors::*;
use crate::claims::Claims;
use crate::errors::TotpEnrollError;
use crate::errors::TotpVerificationError;
use crate::errors::{ClaimValidationError, LoginError, RegisterError};
use derive_more::From;
use model::errors::InternalServerError;
use salvo::http::StatusCode;
use salvo::oapi::{EndpointOutRegister, ToSchema};
use salvo::prelude::*;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TokenResponse {
    #[salvo(schema(value_type = String))]
    #[serde(serialize_with = "crate::models::serialize_secret")]
    pub access_token: SecretString,
    #[salvo(schema(value_type = String))]
    #[serde(serialize_with = "crate::models::serialize_secret")]
    pub refresh_token: SecretString,
    /// E.g. "Bearer"
    pub token_type: String,
    /// Access token expiry in seconds
    pub expires_by: u64,
}

#[derive(From, Debug)]
pub enum RegisterUserResponses {
    Success(TokenResponse),
    BadRequest(BadRegisterUserRequestError),
    UserAlreadyExists,
    InternalServerError(InternalServerError),
}

impl From<Result<TokenResponse, RegisterError>> for RegisterUserResponses {
    fn from(result: Result<TokenResponse, RegisterError>) -> Self {
        match result {
            Ok(token) => token.into(),
            Err(e) => e.into(),
        }
    }
}

impl From<RegisterError> for RegisterUserResponses {
    fn from(e: RegisterError) -> Self {
        match e {
            RegisterError::UserAlreadyExists => Self::UserAlreadyExists,
            RegisterError::BadRequest(e) => Self::BadRequest(e),
            RegisterError::Unexpected(e) => Self::InternalServerError(e),
        }
    }
}

#[async_trait]
impl Writer for RegisterUserResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success(token_response) => {
                res.status_code(StatusCode::CREATED);
                res.render(Json(token_response));
            }
            Self::BadRequest(e) => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(e));
            }
            Self::UserAlreadyExists => {
                res.status_code(StatusCode::CONFLICT);
                res.render(Json(ApiError::new("User already exists")));
            }
            Self::InternalServerError(e) => e.write(req, depot, res).await,
        }
    }
}

impl EndpointOutRegister for RegisterUserResponses {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("201"),
            salvo::oapi::Response::new("Success - user registered and tokens returned")
                .add_content(
                    "application/json",
                    salvo::oapi::Content::new(TokenResponse::to_schema(components)),
                ),
        );
        operation.responses.insert(
            String::from("400"),
            salvo::oapi::Response::new("Bad request - invalid registration data"),
        );
        operation.responses.insert(
            String::from("409"),
            salvo::oapi::Response::new("User already exists"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

#[derive(From, Debug)]
pub enum LoginResponses {
    Success(TokenResponse),
    BadRequest,
    InvalidCredentials,
    InternalServerError(InternalServerError),
}

impl From<Result<TokenResponse, LoginError>> for LoginResponses {
    fn from(result: Result<TokenResponse, LoginError>) -> Self {
        match result {
            Ok(token) => token.into(),
            Err(e) => e.into(),
        }
    }
}

impl From<LoginError> for LoginResponses {
    fn from(e: LoginError) -> Self {
        match e {
            LoginError::InvalidCredentials => Self::InvalidCredentials,
            LoginError::Unexpected(e) => Self::InternalServerError(e),
        }
    }
}

#[async_trait]
impl Writer for LoginResponses {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success(token_response) => {
                res.status_code(StatusCode::OK);
                res.render(Json(token_response));
            }
            Self::BadRequest => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(ApiError::new("Invalid request")));
            }
            Self::InvalidCredentials => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new("Invalid credentials")));
            }
            Self::InternalServerError(e) => {
                e.write(_req, _depot, res).await;
                return;
            }
        }
    }
}

impl EndpointOutRegister for LoginResponses {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("Success - login successful").add_content(
                "application/json",
                salvo::oapi::Content::new(TokenResponse::to_schema(components)),
            ),
        );
        operation.responses.insert(
            String::from("400"),
            salvo::oapi::Response::new("Bad request"),
        );
        operation.responses.insert(
            String::from("401"),
            salvo::oapi::Response::new("Invalid credentials"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

#[derive(From, Debug)]
pub enum RefreshTokenResponses {
    Success(TokenResponse),
    InvalidRefreshToken(String),
    InternalServerError(InternalServerError),
}

impl From<ClaimValidationError> for RefreshTokenResponses {
    fn from(error: ClaimValidationError) -> Self {
        Self::InvalidRefreshToken(error.to_string())
    }
}

impl From<Result<TokenResponse, InternalServerError>> for RefreshTokenResponses {
    fn from(result: Result<TokenResponse, InternalServerError>) -> Self {
        match result {
            Ok(token) => token.into(),
            Err(e) => e.into(),
        }
    }
}

#[async_trait]
impl Writer for RefreshTokenResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success(token_response) => {
                res.status_code(StatusCode::OK);
                res.render(Json(token_response));
            }
            Self::InvalidRefreshToken(e) => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new(e.to_string())));
            }
            Self::InternalServerError(e) => {
                e.write(req, depot, res).await;
                return;
            }
        }
    }
}

impl EndpointOutRegister for RefreshTokenResponses {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("Success - tokens refreshed").add_content(
                "application/json",
                salvo::oapi::Content::new(TokenResponse::to_schema(components)),
            ),
        );
        operation.responses.insert(
            String::from("401"),
            salvo::oapi::Response::new("Invalid or expired refresh token"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

#[derive(From, Debug)]
pub enum ValidateTokenResponses {
    Valid(String),
    Invalid(ClaimValidationError),
}

impl From<Result<Claims, ClaimValidationError>> for ValidateTokenResponses {
    fn from(result: Result<Claims, ClaimValidationError>) -> Self {
        match result {
            Ok(claims) => Self::Valid(claims.sub),
            Err(e) => e.into(),
        }
    }
}

#[async_trait]
impl Writer for ValidateTokenResponses {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Valid(user_id) => {
                res.status_code(StatusCode::OK);
                res.render(Json(user_id));
            }
            Self::Invalid(e) => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new(e.to_string())));
            }
        }
    }
}

impl EndpointOutRegister for ValidateTokenResponses {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("Token is valid - returns user ID"),
        );
        operation.responses.insert(
            String::from("401"),
            salvo::oapi::Response::new("Invalid or expired token"),
        );
    }
}

#[derive(From, Debug)]
pub enum ResetPasswordRequestResponses {
    Success,
    BadRequest,
    InternalServerError(InternalServerError),
}

#[async_trait]
impl Writer for ResetPasswordRequestResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success => {
                res.status_code(StatusCode::OK);
                res.render(Json(ApiError::new("Password reset request sent")));
            }
            Self::BadRequest => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(ApiError::new("Invalid request")));
            }
            Self::InternalServerError(e) => e.write(req, depot, res).await,
        }
    }
}

impl EndpointOutRegister for ResetPasswordRequestResponses {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("Password reset email sent (if user exists)"),
        );
        operation.responses.insert(
            String::from("400"),
            salvo::oapi::Response::new("Bad request"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

#[derive(From, Debug)]
pub enum PasswordResetResponses {
    Success,
    InvalidToken,
    InvalidNewPassword,
    InternalServerError(InternalServerError),
}

#[async_trait]
impl Writer for PasswordResetResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success => {
                res.status_code(StatusCode::OK);
                res.render(Json(ApiError::new("Password reset successful")));
            }
            Self::InvalidToken => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(ApiError::new("Invalid or expired token")));
            }
            Self::InvalidNewPassword => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(ApiError::new("Invalid new password")));
            }
            Self::InternalServerError(e) => e.write(req, depot, res).await,
        }
    }
}

impl EndpointOutRegister for PasswordResetResponses {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("Password reset successful"),
        );
        operation.responses.insert(
            String::from("400"),
            salvo::oapi::Response::new("Invalid or expired token, or invalid new password"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

#[derive(From, Debug)]
pub enum UserProfileResponses {
    Success(UserProfile),
    Unauthorized(ClaimValidationError),
    UserNotFound,
    InternalServerError(InternalServerError),
}

#[async_trait]
impl Writer for UserProfileResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success(user_profile) => {
                res.status_code(StatusCode::OK);
                res.render(Json(user_profile));
            }
            Self::Unauthorized(e) => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new(e.to_string())));
            }
            Self::UserNotFound => {
                res.status_code(StatusCode::NOT_FOUND);
                res.render(Json(ApiError::new("User not found")));
            }
            Self::InternalServerError(e) => e.write(req, depot, res).await,
        }
    }
}

impl EndpointOutRegister for UserProfileResponses {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("Success - returns user profile").add_content(
                "application/json",
                salvo::oapi::Content::new(UserProfile::to_schema(components)),
            ),
        );
        operation.responses.insert(
            String::from("401"),
            salvo::oapi::Response::new("Unauthorized - invalid or missing token"),
        );
        operation.responses.insert(
            String::from("404"),
            salvo::oapi::Response::new("User not found"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

pub enum UpdateUserProfileResponses {
    Success(UserProfile),
    BadRequest,
    Unauthorized(ClaimValidationError),
    InvalidPassword,
    UserNotFound,
    InternalServerError(InternalServerError),
}

#[async_trait]
impl Writer for UpdateUserProfileResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success(profile) => {
                res.status_code(StatusCode::OK);
                res.render(Json(profile));
            }
            Self::BadRequest => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(ApiError::new("Invalid request")));
            }
            Self::Unauthorized(e) => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new(e.to_string())));
            }
            Self::InvalidPassword => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(ApiError::new("Invalid password")));
            }
            Self::UserNotFound => {
                res.status_code(StatusCode::NOT_FOUND);
                res.render(Json(ApiError::new("User not found")));
            }
            Self::InternalServerError(e) => e.write(req, depot, res).await,
        }
    }
}

impl EndpointOutRegister for UpdateUserProfileResponses {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("Success - returns updated user profile").add_content(
                "application/json",
                salvo::oapi::Content::new(UserProfile::to_schema(components)),
            ),
        );
        operation.responses.insert(
            String::from("400"),
            salvo::oapi::Response::new("Invalid request or password"),
        );
        operation.responses.insert(
            String::from("401"),
            salvo::oapi::Response::new("Unauthorized - invalid or missing token"),
        );
        operation.responses.insert(
            String::from("404"),
            salvo::oapi::Response::new("User not found"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

#[derive(From)]
pub enum LogoutResponses {
    Success,
    Unauthorized(ClaimValidationError),
    InternalServerError(InternalServerError),
}

#[async_trait]
impl Writer for LogoutResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success => {
                res.status_code(StatusCode::OK);
                res.render(Json(ApiError::new("Logout successful")));
            }
            Self::Unauthorized(e) => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new(e.to_string())));
            }
            Self::InternalServerError(e) => e.write(req, depot, res).await,
        }
    }
}

impl EndpointOutRegister for LogoutResponses {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("Logout successful"),
        );
        operation.responses.insert(
            String::from("401"),
            salvo::oapi::Response::new("Unauthorized - invalid or missing token"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

// TOTP Response types

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TotpEnrollmentResponse {
    pub provisioning_uri: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct MfaRequiredResponse {
    #[salvo(schema(value_type = String))]
    #[serde(serialize_with = "crate::models::serialize_secret")]
    pub mfa_token: SecretString,
    pub message: String,
}

#[derive(From, Debug)]
pub enum TotpEnrollResponses {
    Success(TotpEnrollmentResponse),
    AlreadyEnabled,
    Unauthorized(ClaimValidationError),
    InternalServerError(InternalServerError),
}

impl From<Result<TotpEnrollmentResponse, TotpEnrollError>> for TotpEnrollResponses {
    fn from(result: Result<TotpEnrollmentResponse, TotpEnrollError>) -> Self {
        match result {
            Ok(response) => response.into(),
            Err(e) => e.into(),
        }
    }
}

impl From<TotpEnrollError> for TotpEnrollResponses {
    fn from(e: TotpEnrollError) -> Self {
        match e {
            TotpEnrollError::AlreadyEnabled => Self::AlreadyEnabled,
            TotpEnrollError::UserNotFound => {
                Self::InternalServerError(eyre::eyre!("User not found").into())
            }
            TotpEnrollError::Db(e) => Self::InternalServerError(e.into()),
            TotpEnrollError::Unexpected(e) => Self::InternalServerError(e.into()),
        }
    }
}

#[async_trait]
impl Writer for TotpEnrollResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success(enrollment) => {
                res.status_code(StatusCode::OK);
                res.render(Json(enrollment));
            }
            Self::AlreadyEnabled => {
                res.status_code(StatusCode::CONFLICT);
                res.render(Json(ApiError::new("TOTP is already enabled")));
            }
            Self::Unauthorized(e) => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new(e.to_string())));
            }
            Self::InternalServerError(e) => e.write(req, depot, res).await,
        }
    }
}

impl EndpointOutRegister for TotpEnrollResponses {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("Success - TOTP enrollment initiated").add_content(
                "application/json",
                salvo::oapi::Content::new(TotpEnrollmentResponse::to_schema(components)),
            ),
        );
        operation.responses.insert(
            String::from("401"),
            salvo::oapi::Response::new("Unauthorized"),
        );
        operation.responses.insert(
            String::from("409"),
            salvo::oapi::Response::new("TOTP already enabled"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

#[derive(From, Debug)]
pub enum TotpVerifyEnrollmentResponses {
    Success,
    InvalidCode,
    NotEnrolled,
    Unauthorized(ClaimValidationError),
    InternalServerError(InternalServerError),
}

impl From<Result<(), TotpVerificationError>> for TotpVerifyEnrollmentResponses {
    fn from(result: Result<(), TotpVerificationError>) -> Self {
        match result {
            Ok(()) => Self::Success,
            Err(e) => e.into(),
        }
    }
}

impl From<TotpVerificationError> for TotpVerifyEnrollmentResponses {
    fn from(e: TotpVerificationError) -> Self {
        match e {
            TotpVerificationError::UserNotFound => {
                Self::InternalServerError(eyre::eyre!("User not found").into())
            }
            TotpVerificationError::InvalidCode => Self::InvalidCode,
            TotpVerificationError::NotEnabled => Self::NotEnrolled,
            TotpVerificationError::Unexpected(e) => Self::InternalServerError(e.into()),
        }
    }
}

#[async_trait]
impl Writer for TotpVerifyEnrollmentResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success => {
                res.status_code(StatusCode::OK);
                res.render(Json(ApiError::new("TOTP enabled successfully")));
            }
            Self::InvalidCode => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(ApiError::new("Invalid TOTP code")));
            }
            Self::NotEnrolled => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(ApiError::new("TOTP enrollment not initiated")));
            }
            Self::Unauthorized(e) => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new(e.to_string())));
            }
            Self::InternalServerError(e) => e.write(req, depot, res).await,
        }
    }
}

impl EndpointOutRegister for TotpVerifyEnrollmentResponses {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("TOTP enabled successfully"),
        );
        operation.responses.insert(
            String::from("400"),
            salvo::oapi::Response::new("Invalid TOTP code or enrollment not initiated"),
        );
        operation.responses.insert(
            String::from("401"),
            salvo::oapi::Response::new("Unauthorized"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

#[derive(From, Debug)]
pub enum TotpVerifyLoginResponses {
    Success(TokenResponse),
    InvalidMfaToken,
    InvalidCode,
    NotEnrolled,
    MaxAttemptsExceeded,
    InternalServerError(InternalServerError),
}

impl From<Result<TokenResponse, TotpVerificationError>> for TotpVerifyLoginResponses {
    fn from(result: Result<TokenResponse, TotpVerificationError>) -> Self {
        match result {
            Ok(tokens) => tokens.into(),
            Err(e) => e.into(),
        }
    }
}

impl From<TotpVerificationError> for TotpVerifyLoginResponses {
    fn from(e: TotpVerificationError) -> Self {
        match e {
            TotpVerificationError::UserNotFound => {
                Self::InternalServerError(eyre::eyre!("User not found").into())
            }
            TotpVerificationError::NotEnabled => Self::NotEnrolled,
            TotpVerificationError::InvalidCode => Self::InvalidCode,
            TotpVerificationError::Unexpected(e) => Self::InternalServerError(e.into()),
        }
    }
}

#[async_trait]
impl Writer for TotpVerifyLoginResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success(tokens) => {
                res.status_code(StatusCode::OK);
                res.render(Json(tokens));
            }
            Self::InvalidMfaToken => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new("Invalid MFA token")));
            }
            Self::InvalidCode => {
                res.status_code(StatusCode::FORBIDDEN);
                res.render(Json(ApiError::new("Invalid TOTP code")));
            }
            Self::NotEnrolled => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(ApiError::new("TOTP not enrolled")));
            }
            Self::MaxAttemptsExceeded => {
                res.status_code(StatusCode::TOO_MANY_REQUESTS);
                res.render(Json(ApiError::new(
                    "Maximum verification attempts exceeded",
                )));
            }
            Self::InternalServerError(e) => e.write(req, depot, res).await,
        }
    }
}

impl EndpointOutRegister for TotpVerifyLoginResponses {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("Success - login completed").add_content(
                "application/json",
                salvo::oapi::Content::new(TokenResponse::to_schema(components)),
            ),
        );
        operation.responses.insert(
            String::from("401"),
            salvo::oapi::Response::new("Invalid or expired MFA token"),
        );
        operation.responses.insert(
            String::from("403"),
            salvo::oapi::Response::new("Invalid TOTP code"),
        );
        operation.responses.insert(
            String::from("400"),
            salvo::oapi::Response::new("TOTP not enrolled"),
        );
        operation.responses.insert(
            String::from("429"),
            salvo::oapi::Response::new("Maximum verification attempts exceeded"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}

#[derive(From, Debug)]
pub enum TotpDisableResponses {
    Success,
    NotEnrolled,
    Unauthorized(ClaimValidationError),
    InvalidCode,
    InternalServerError(InternalServerError),
}

impl From<Result<(), TotpVerificationError>> for TotpDisableResponses {
    fn from(result: Result<(), TotpVerificationError>) -> Self {
        match result {
            Ok(()) => Self::Success,
            Err(e) => e.into(),
        }
    }
}

impl From<TotpVerificationError> for TotpDisableResponses {
    fn from(e: TotpVerificationError) -> Self {
        match e {
            TotpVerificationError::UserNotFound => {
                Self::InternalServerError(eyre::eyre!("User not found").into())
            }
            TotpVerificationError::NotEnabled => Self::NotEnrolled,
            TotpVerificationError::InvalidCode => Self::InvalidCode,
            TotpVerificationError::Unexpected(e) => Self::InternalServerError(e.into()),
        }
    }
}

#[async_trait]
impl Writer for TotpDisableResponses {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Success => {
                res.status_code(StatusCode::OK);
                res.render(Json(ApiError::new("TOTP enabled successfully")));
            }
            Self::NotEnrolled => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(Json(ApiError::new("TOTP enrollment not initiated")));
            }
            Self::Unauthorized(e) => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new(e.to_string())));
            }
            Self::InvalidCode => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(ApiError::new("Invalid TOTP code")));
            }
            Self::InternalServerError(e) => e.write(req, depot, res).await,
        }
    }
}

impl EndpointOutRegister for TotpDisableResponses {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            String::from("200"),
            salvo::oapi::Response::new("TOTP enabled successfully"),
        );
        operation.responses.insert(
            String::from("400"),
            salvo::oapi::Response::new("Invalid TOTP code or enrollment not initiated"),
        );
        operation.responses.insert(
            String::from("401"),
            salvo::oapi::Response::new("Invalid or expired MFA token"),
        );
        operation.responses.insert(
            String::from("500"),
            salvo::oapi::Response::new("Internal server error"),
        );
    }
}
