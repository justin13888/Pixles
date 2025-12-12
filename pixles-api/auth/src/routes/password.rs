use axum::Json;
use axum::extract::State;
use docs::TAGS;
use service::user as UserService;
use tracing::trace;

use crate::models::responses::{PasswordResetResponses, ResetPasswordRequestResponses};
use crate::models::{ResetPasswordPayload, ResetPasswordRequestPayload};
use crate::state::AppState;
use crate::utils::hash::hash_password;

/// Request password reset
#[utoipa::path(
    post,
    path = "/password-reset-request",
    tag = TAGS::AUTH,
    security(),
    request_body = ResetPasswordRequestPayload,
    responses(ResetPasswordRequestResponses),
    tags = ["Pixles Authentication API"]
)]
pub async fn reset_password_request(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequestPayload>,
) -> ResetPasswordRequestResponses {
    let email = payload.email;

    if let Err(e) = state
        .password_service
        .request_reset(&state.conn, &state.email_service, &email)
        .await
    {
        return ResetPasswordRequestResponses::InternalServerError(e.into());
    }

    ResetPasswordRequestResponses::Success
}

/// Reset password with token
#[utoipa::path(
    post,
    path = "/password-reset",
    tag = TAGS::AUTH,
    security(),
    request_body = ResetPasswordPayload,
    responses(PasswordResetResponses),
    tags = ["Pixles Authentication API"]
)]
pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordPayload>,
) -> PasswordResetResponses {
    let ResetPasswordPayload {
        token,
        new_password,
    } = payload;

    // Find user by token
    let user = match UserService::Query::find_user_by_reset_token(&state.conn, &token).await {
        Ok(user) => user,
        Err(e) => return PasswordResetResponses::InternalServerError(e.into()),
    };

    let user = match user {
        Some(user) => user,
        None => return PasswordResetResponses::InvalidToken,
    };

    // Check expiry
    if let Some(expires_at) = user.password_reset_expires_at {
        if expires_at < chrono::Utc::now() {
            return PasswordResetResponses::InvalidToken;
        }
    } else {
        return PasswordResetResponses::InvalidToken;
    }

    // Validate new password strength
    if let Err(e) = UserService::is_valid_password(&new_password) {
        trace!("Invalid password during reset for user {}: {}", user.id, e);
        return PasswordResetResponses::InvalidNewPassword;
    }

    // Hash password
    let password_hash = match hash_password(&new_password) {
        Ok(hash) => hash,
        Err(e) => return PasswordResetResponses::InternalServerError(e.into()),
    };

    // Confirm reset
    if let Err(e) =
        UserService::Mutation::confirm_password_reset(&state.conn, user.id.clone(), password_hash)
            .await
    {
        return PasswordResetResponses::InternalServerError(e.into());
    }

    // Revoke all existing sessions for security
    if let Err(e) = state.session_manager.revoke_all_for_user(&user.id).await {
        tracing::error!(
            "Failed to revoke sessions after password reset for user {}: {}",
            user.id,
            e
        );
    }

    PasswordResetResponses::Success
}
