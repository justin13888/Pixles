mod auth;
mod password;
mod profile;

use aide::axum::routing::get;
use aide::axum::{ApiRouter, routing::post};
use docs::TAGS;

use crate::state::AppState;

pub(super) fn get_router(state: AppState) -> ApiRouter {
    ApiRouter::new()
        // Profile routes
        .api_route_with(
            "/profile",
            get(profile::get_user_profile).post(profile::update_user_profile),
            |op| op.tag(TAGS::AUTH),
        )
        // Auth routes
        .api_route_with("/register", post(auth::register_user), |op| {
            op.tag(TAGS::AUTH)
                .description("Register a new user")
                .security_requirement("bearer")
        })
        .api_route_with("/login", post(auth::login_user), |op| {
            op.tag(TAGS::AUTH).description("Login a user")
        })
        .api_route_with("/refresh", post(auth::refresh_token), |op| {
            op.tag(TAGS::AUTH)
                .description("Refresh an access token using a refresh token")
        })
        .api_route_with("/validate", post(auth::validate_token), |op| {
            op.tag(TAGS::AUTH).description("Validate an access token")
        })
        .api_route_with("/logout", post(auth::logout), |op| {
            op.tag(TAGS::AUTH)
                .description("Logout user and invalidate tokens")
                .security_requirement("bearer")
        })
        // Password routes
        .api_route_with(
            "/password-reset-request",
            post(password::reset_password_request),
            |op| op.tag(TAGS::AUTH).description("Request password reset"),
        )
        .api_route_with("/password-reset", post(password::reset_password), |op| {
            op.tag(TAGS::AUTH).description("Reset password with token")
        })
        .with_state(state)
}

// TODO: Alerting
// - Multiple failed login attempts
// - Unusual authentication patterns
// - Rate limit threshold breaches
