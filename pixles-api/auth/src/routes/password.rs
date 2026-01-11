use model::errors::InternalServerError;
use salvo::oapi::extract::JsonBody;
use salvo::prelude::*;
use service::user as UserService;
use tracing::trace;

use crate::models::responses::{PasswordResetResponses, ResetPasswordRequestResponses};
use crate::models::{ResetPasswordPayload, ResetPasswordRequestPayload};
use crate::state::AppState;
use crate::utils::hash::hash_password;

/// Request password reset
#[endpoint(operation_id = "reset_password_request", tags("auth"))]
pub async fn reset_password_request(
    depot: &mut Depot,
    body: JsonBody<ResetPasswordRequestPayload>,
) -> ResetPasswordRequestResponses {
    let state = depot.obtain::<AppState>().unwrap();
    let email = body.into_inner().email;

    if let Err(e) = state
        .password_service
        .request_reset(&state.conn, &state.email_service, &email)
        .await
    {
        return ResetPasswordRequestResponses::InternalServerError(eyre::eyre!(e).into());
    }

    ResetPasswordRequestResponses::Success
}

/// Reset password with token
#[endpoint(operation_id = "reset_password", tags("auth"))]
pub async fn reset_password(
    depot: &mut Depot,
    body: JsonBody<ResetPasswordPayload>,
) -> PasswordResetResponses {
    let state = depot.obtain::<AppState>().unwrap();

    let ResetPasswordPayload {
        token,
        new_password,
    } = body.into_inner();

    // Find user by token
    let user = match UserService::Query::find_user_by_reset_token(&state.conn, &token).await {
        Ok(user) => user,
        Err(e) => return PasswordResetResponses::InternalServerError(eyre::eyre!(e).into()),
    };

    let user = match user {
        Some(user) => user,
        None => return PasswordResetResponses::InvalidToken,
    };

    // Check expiry
    if user
        .password_reset_expires_at
        .is_none_or(|exp| exp < chrono::Utc::now())
    {
        return PasswordResetResponses::InvalidToken;
    }

    // Validate new password strength
    if !UserService::is_valid_password(&new_password) {
        trace!("Invalid password during reset for user {}", user.id);
        return PasswordResetResponses::InvalidNewPassword;
    }

    // Hash password
    let password_hash = match hash_password(&new_password) {
        Ok(hash) => hash,
        Err(e) => return PasswordResetResponses::InternalServerError(eyre::eyre!(e).into()),
    };

    // Confirm reset
    if let Err(e) =
        UserService::Mutation::confirm_password_reset(&state.conn, user.id.clone(), password_hash)
            .await
    {
        return PasswordResetResponses::InternalServerError(eyre::eyre!(e).into());
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
