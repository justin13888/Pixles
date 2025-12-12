mod auth;
mod password;
mod profile;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

pub(super) fn get_router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(
            profile::get_user_profile,
            profile::update_user_profile
        )) // Profile routes (/profile)
        .routes(routes!(auth::register_user)) // POST /register
        .routes(routes!(auth::login_user)) // POST /login
        .routes(routes!(auth::refresh_token)) // POST /refresh
        .routes(routes!(auth::validate_token)) // POST /validate
        .routes(routes!(password::reset_password_request)) // POST /password-reset-request
        .routes(routes!(password::reset_password)) // POST /password-reset
        .routes(routes!(auth::logout)) // POST /logout
        .with_state(state)
}

// TODO: Alerting
// - Multiple failed login attempts
// - Unusual authentication patterns
// - Rate limit threshold breaches
