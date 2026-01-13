use salvo::oapi::extract::JsonBody;
use salvo::prelude::*;
use secrecy::ExposeSecret;

use crate::claims::Claims;
use crate::errors::ClaimValidationError;
use crate::models::requests::{LoginRequest, RefreshTokenRequest, RegisterRequest};
use crate::models::responses::{
    GetDevicesResponses, LoginResponses, LogoutResponses, RefreshTokenResponses,
    RegisterUserResponses, ValidateTokenResponses,
};
use crate::state::AppState;
use crate::utils::headers::get_token_from_headers;

/// Register a new user
#[endpoint(operation_id = "register_user", tags("auth"))]
pub async fn register_user(
    depot: &mut Depot,
    body: JsonBody<RegisterRequest>,
) -> RegisterUserResponses {
    let state = depot.obtain::<AppState>().unwrap();
    let request = body.into_inner();

    state
        .auth_service
        .register_user(&state.session_manager, request)
        .await
        .into()
}

/// Login a user
#[endpoint(operation_id = "login_user", tags("auth"))]
pub async fn login_user(depot: &mut Depot, body: JsonBody<LoginRequest>) -> LoginResponses {
    let state = depot.obtain::<AppState>().unwrap();
    let LoginRequest { email, password } = body.into_inner();

    state
        .auth_service
        .authenticate_user(&state.session_manager, &email, &password)
        .await
        .into()
}

/// Refresh an access token using a refresh token
#[endpoint(operation_id = "refresh_token", tags("auth"))]
pub async fn refresh_token(
    depot: &mut Depot,
    body: JsonBody<RefreshTokenRequest>,
) -> RefreshTokenResponses {
    let state = depot.obtain::<AppState>().unwrap();
    let payload = body.into_inner();

    let token = match Claims::decode(
        payload.refresh_token.expose_secret(),
        &state.config.jwt_eddsa_decoding_key,
    ) {
        Ok(token) => token,
        Err(e) => {
            return ClaimValidationError::from(e).into();
        }
    };

    let sid = match token.claims.sid {
        Some(sid) => sid,
        None => {
            return ClaimValidationError::TokenInvalid("Missing session ID".to_string()).into();
        }
    };

    // Validate session
    let session = match state.session_manager.get_session(&sid).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return ClaimValidationError::TokenInvalid("Session not found".to_string()).into();
        }
        Err(e) => return e.into(),
    };

    // Verify user ownership
    if session.user_id != token.claims.sub {
        return ClaimValidationError::TokenInvalid("Session user mismatch".to_string()).into();
    }

    // Revoke old session (Rotation)
    if let Err(e) = state.session_manager.revoke_session(&sid).await {
        return e.into();
    }

    let user_id = token.claims.sub;

    state
        .auth_service
        .generate_token_pair(&user_id, &state.session_manager)
        .await
        .into()
}

/// Validate an access token
#[endpoint(operation_id = "validate_token", tags("auth"), security(("bearer" = [])))]
pub async fn validate_token(req: &mut Request, depot: &mut Depot) -> ValidateTokenResponses {
    let state = depot.obtain::<AppState>().unwrap();
    let headers = req.headers();

    // Get token string
    let token_string = match get_token_from_headers(headers) {
        Ok(token_string) => token_string,
        Err(e) => return e.into(),
    };

    // Validate token
    let claims = match state.auth_service.get_claims(token_string.expose_secret()) {
        Ok(claims) => claims,
        Err(e) => return e.into(),
    };

    if let Err(e) = claims.validate_access_token() {
        return e.into();
    }

    Ok(claims).into()
}

/// Logout user and invalidate tokens
#[endpoint(operation_id = "logout", tags("auth"), security(("bearer" = [])))]
pub async fn logout(req: &mut Request, depot: &mut Depot) -> LogoutResponses {
    let state = depot.obtain::<AppState>().unwrap();
    let headers = req.headers();

    // Authorize user
    let token_string = match get_token_from_headers(headers) {
        Ok(token_string) => token_string,
        Err(e) => return e.into(),
    };

    // Just validate the token signatures
    let claims = match state.auth_service.get_claims(token_string.expose_secret()) {
        Ok(claims) => claims,
        Err(e) => return e.into(),
    };

    if let Err(e) = claims.validate_access_token() {
        return e.into();
    }

    // If it has a session ID, revoke it
    if let Some(sid) = claims.sid
        && let Err(e) = state.session_manager.revoke_session(&sid).await
    {
        return e.into();
    }

    LogoutResponses::Success
}

/// Get all active devices (sessions)
#[endpoint(operation_id = "get_devices", tags("auth"), security(("bearer" = [])))]
pub async fn get_devices(req: &mut Request, depot: &mut Depot) -> GetDevicesResponses {
    todo!("Implement get_devices")
}
