mod auth;
mod password;
mod profile;

use salvo::affix_state;
use salvo::prelude::*;

use crate::state::AppState;

pub(super) fn get_router(state: AppState) -> Router {
    Router::new()
        // Inject state into depot for all routes
        .hoop(affix_state::inject(state))
        // Profile routes
        .push(
            Router::with_path("profile")
                .get(profile::get_user_profile)
                .post(profile::update_user_profile),
        )
        // Auth routes
        .push(Router::with_path("register").post(auth::register_user))
        .push(Router::with_path("login").post(auth::login_user))
        .push(Router::with_path("refresh").post(auth::refresh_token))
        .push(Router::with_path("validate").post(auth::validate_token))
        .push(Router::with_path("logout").post(auth::logout))
        // Password routes
        .push(Router::with_path("password-reset-request").post(password::reset_password_request))
        .push(Router::with_path("password-reset").post(password::reset_password))
}

// TODO: Alerting
// - Multiple failed login attempts
// - Unusual authentication patterns
// - Rate limit threshold breaches
