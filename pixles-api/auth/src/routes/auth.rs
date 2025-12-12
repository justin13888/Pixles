use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use secrecy::ExposeSecret;

use crate::claims::Claims;
use crate::errors::ClaimValidationError;
use crate::models::requests::{LoginRequest, RefreshTokenRequest, RegisterRequest};
use crate::models::responses::{
    LoginResponses, LogoutResponses, RefreshTokenResponses, RegisterUserResponses,
    ValidateTokenResponses,
};
use crate::state::AppState;
use crate::utils::headers::get_token_from_headers;

/// Register a new user
pub async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> RegisterUserResponses {
    state
        .auth_service
        .register_user(&state.conn, &state.session_manager, request)
        .await
        .into()
}

/// Login a user
pub async fn login_user(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> LoginResponses {
    let LoginRequest { email, password } = request;

    state
        .auth_service
        .authenticate_user(&state.conn, &state.session_manager, &email, &password)
        .await
        .into()
}

/// Refresh an access token using a refresh token
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> RefreshTokenResponses {
    let token = match Claims::decode(
        payload.refresh_token.expose_secret(),
        &state.config.jwt_eddsa_decoding_key,
    ) {
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

    state
        .auth_service
        .generate_token_pair(&user_id, &state.session_manager)
        .await
        .into()
}

/// Validate an access token
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
    state
        .auth_service
        .get_claims(token_string.expose_secret())
        .into()
}

/// Logout user and invalidate tokens
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
