use axum::Json;
use axum::extract::State;
use docs::TAGS;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::models::responses::{PasswordResetResponses, ResetPasswordRequestResponses};
use crate::models::{ResetPasswordPayload, ResetPasswordRequestPayload};
use crate::state::AppState;

/// Request password reset
#[utoipa::path(
    post,
    path = "/password-reset-request",
    tag = TAGS::AUTH,
    request_body = ResetPasswordRequestPayload,
    responses(ResetPasswordRequestResponses),
    tags = ["Pixles Authentication API"]
)]
pub async fn reset_password_request(
    State(AppState { config, .. }): State<AppState>,
    Json(payload): Json<ResetPasswordRequestPayload>,
) -> ResetPasswordRequestResponses {
    // In a real implementation, you would:
    // 1. Check if user exists
    // 2. Generate a password reset token
    // 3. Store token in database with expiry
    // 4. Send email with reset link

    // TODO: Implement reset password request
    // Needs DB schema update to store reset tokens.

    // For this example, we'll simulate successful request
    // In real app, check if email exists in database
    if payload.email.contains("example.com") { // Simple mock check
        // In real app, send email with reset link
    } else {
        // TODO: Ensure it doesn't leak if email exists with consistent response time
    }

    ResetPasswordRequestResponses::Success
}

/// Reset password with token
#[utoipa::path(
    post,
    path = "/password-reset",
    tag = TAGS::AUTH,
    request_body = ResetPasswordPayload,
    responses(PasswordResetResponses),
    tags = ["Pixles Authentication API"]
)]
pub async fn reset_password(
    State(AppState { config, .. }): State<AppState>,
    Json(payload): Json<ResetPasswordPayload>,
) -> PasswordResetResponses {
    // In a real implementation, you would:
    // 1. Validate reset token from database
    // 2. Check if token is expired
    // 3. Update user's password
    // 4. Invalidate the reset token

    // TODO: Implement password reset
    // Needs DB schema update.

    // For this example, we'll simulate successful password reset
    if payload.token == "valid_reset_token" {
        PasswordResetResponses::Success
    } else {
        PasswordResetResponses::InvalidToken
    }
}
