mod auth;
mod password;
mod profile;

use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with};
use docs::TAGS;

use crate::state::AppState;

pub(super) fn get_router(state: AppState) -> ApiRouter {
    ApiRouter::new()
        // Profile routes
        .api_route_with(
            "/profile",
            get_with(profile::get_user_profile, |op| op.id("get_user_profile"))
                .post_with(profile::update_user_profile, |op| {
                    op.id("update_user_profile")
                }),
            |op| op.tag(TAGS::AUTH),
        )
        // Auth routes
        .api_route_with(
            "/register",
            post_with(auth::register_user, |op| op.id("register_user")),
            |op| {
                op.tag(TAGS::AUTH)
                    .description("Register a new user")
                    .security_requirement("bearer")
            },
        )
        .api_route_with(
            "/login",
            post_with(auth::login_user, |op| op.id("login_user")),
            |op| op.tag(TAGS::AUTH).description("Login a user"),
        )
        .api_route_with(
            "/refresh",
            post_with(auth::refresh_token, |op| op.id("refresh_token")),
            |op| {
                op.tag(TAGS::AUTH)
                    .description("Refresh an access token using a refresh token")
            },
        )
        .api_route_with(
            "/validate",
            post_with(auth::validate_token, |op| op.id("validate_token")),
            |op| op.tag(TAGS::AUTH).description("Validate an access token"),
        )
        .api_route_with(
            "/logout",
            post_with(auth::logout, |op| op.id("logout")),
            |op| {
                op.tag(TAGS::AUTH)
                    .description("Logout user and invalidate tokens")
                    .security_requirement("bearer")
            },
        )
        // Password routes
        .api_route_with(
            "/password-reset-request",
            post_with(password::reset_password_request, |op| {
                op.id("reset_password_request")
            }),
            |op| op.tag(TAGS::AUTH).description("Request password reset"),
        )
        .api_route_with(
            "/password-reset",
            post_with(password::reset_password, |op| op.id("reset_password")),
            |op| op.tag(TAGS::AUTH).description("Reset password with token"),
        )
        .with_state(state)
}

// TODO: Alerting
// - Multiple failed login attempts
// - Unusual authentication patterns
// - Rate limit threshold breaches
