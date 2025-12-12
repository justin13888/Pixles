use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use docs::TAGS;
use secrecy::ExposeSecret;
use tracing::trace;

use crate::claims::Claims;
use crate::errors::{AuthError, ClaimValidationError};
use crate::models::requests::{LoginRequest, RefreshTokenRequest, RegisterRequest};
use crate::models::responses::{
    LoginResponses, LogoutResponses, RefreshTokenResponses, RegisterUserResponses,
    ValidateTokenResponse, ValidateTokenResponses,
};
use crate::state::AppState;
use crate::utils::headers::get_token_from_headers;

/// Register a new user
#[utoipa::path(
    post,
    path = "/register",
    tag = TAGS::AUTH,
    request_body = RegisterRequest,
    responses(RegisterUserResponses),
    tags = ["Pixles Authentication API"]
)]
pub async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> RegisterUserResponses {
    match state
        .auth_service
        .register_user(&state.conn, &state.session_manager, request)
        .await
    {
        Ok(token_response) => RegisterUserResponses::Success(token_response),
        Err(AuthError::UserAlreadyExists) => RegisterUserResponses::UserAlreadyExists,
        Err(AuthError::BadRequest(e)) => RegisterUserResponses::BadRequest(e),
        Err(e) => RegisterUserResponses::InternalServerError(e.into()),
    }
}

/// Login a user
#[utoipa::path(
    post,
    path = "/login",
    tag = TAGS::AUTH,
    request_body = LoginRequest,
    responses(LoginResponses),
    tags = ["Pixles Authentication API"]
)]
pub async fn login_user(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> LoginResponses {
    let LoginRequest { email, password } = request;

    match state
        .auth_service
        .authenticate_user(&state.conn, &state.session_manager, &email, &password)
        .await
    {
        Ok(token_response) => LoginResponses::Success(token_response),
        Err(AuthError::InvalidCredentials) => LoginResponses::InvalidCredentials,
        Err(e) => LoginResponses::InternalServerError(e.into()),
    }
}

/// Refresh an access token using a refresh token
#[utoipa::path(
    post,
    path = "/refresh",
    tag = TAGS::AUTH,
    request_body = RefreshTokenRequest,
    responses(RefreshTokenResponses),
    tags = ["Pixles Authentication API"]
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> RefreshTokenResponses {
    let token = match Claims::decode(&payload.refresh_token, &state.config.jwt_eddsa_decoding_key) {
        Ok(token) => token,
        Err(e) => {
            return RefreshTokenResponses::InvalidRefreshToken(
                ClaimValidationError::from(e).into(),
            );
        }
    };

    let sid = match token.claims.sid {
        Some(sid) => sid,
        None => {
            return RefreshTokenResponses::InvalidRefreshToken(
                ClaimValidationError::TokenInvalid("Missing session ID".to_string()).into(),
            );
        }
    };

    // Validate session
    let session = match state.session_manager.get_session(&sid).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return RefreshTokenResponses::InvalidRefreshToken(
                ClaimValidationError::TokenInvalid("Session not found".to_string()).into(),
            );
        }
        Err(e) => return RefreshTokenResponses::InternalServerError(e.into()),
    };

    // Verify user ownership
    if session.user_id != token.claims.sub {
        return RefreshTokenResponses::InvalidRefreshToken(
            ClaimValidationError::TokenInvalid("Session user mismatch".to_string()).into(),
        );
    }

    // Revoke old session (Rotation)
    if let Err(e) = state.session_manager.revoke_session(&sid).await {
        return RefreshTokenResponses::InternalServerError(e.into());
    }

    let user_id = token.claims.sub;

    match state
        .auth_service
        .generate_token_pair(&user_id, &state.session_manager)
        .await
    {
        Ok(token_response) => RefreshTokenResponses::Success(token_response),
        Err(e) => RefreshTokenResponses::InternalServerError(e.into()),
    }
}

/// Validate an access token
#[utoipa::path(
    post,
    path = "/validate",
    tag = TAGS::AUTH,
    security(
        ("bearer" = [])
    ),
    responses(ValidateTokenResponses),
    tags = ["Pixles Authentication API"]
)]
pub async fn validate_token(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> ValidateTokenResponses {
    // Get token string
    let token_string = match get_token_from_headers(&headers) {
        Ok(token_string) => token_string,
        Err(e) => return ValidateTokenResponses::Invalid(e.into()),
    };

    // Validate token
    match state.auth_service.get_claims(token_string.expose_secret()) {
        Ok(claims) => ValidateTokenResponses::Valid(ValidateTokenResponse::Valid(claims.sub)),
        Err(e) => ValidateTokenResponses::Invalid(e.into()),
    }
}

/// Logout user and invalidate tokens
#[utoipa::path(
    post,
    path = "/logout",
    tag = TAGS::AUTH,
    security(
        ("bearer" = [])
    ),
    responses(LogoutResponses),
    tags = ["Pixles Authentication API"]
)]
pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> LogoutResponses {
    // Authorize user
    let token_string = match get_token_from_headers(&headers) {
        Ok(token_string) => token_string,
        Err(e) => return LogoutResponses::Unauthorized(e.into()),
    };

    // Just validate the token signatures
    let claims = match state.auth_service.get_claims(token_string.expose_secret()) {
        Ok(claims) => claims,
        Err(e) => return LogoutResponses::Unauthorized(e.into()),
    };

    // If it has a session ID, revoke it
    if let Some(sid) = claims.sid
        && let Err(e) = state.session_manager.revoke_session(&sid).await
    {
        return LogoutResponses::InternalServerError(e.into());
    }

    LogoutResponses::Success
}
