use aide::axum::ApiRouter;
use config::AuthConfig;
use eyre::Result;
use sea_orm::DatabaseConnection;
use state::AppState;
use tower_http::cors::{Any, CorsLayer};

pub mod claims;
pub mod config;
pub mod constants;
pub mod errors;
#[cfg(feature = "server")]
mod models;
pub mod oidc;
pub mod roles;
#[cfg(feature = "server")]
mod routes;
pub mod service;
#[cfg(feature = "server")]
pub mod session;
#[cfg(feature = "server")]
mod state;
pub mod utils;
#[cfg(feature = "server")]
pub mod validation;

#[cfg(feature = "server")]
pub async fn get_router<C: Into<AuthConfig>>(
    conn: DatabaseConnection,
    config: C,
) -> Result<ApiRouter> {
    let config = config.into();

    let session_manager = session::SessionManager::new(
        config.valkey_url.clone(),
        std::time::Duration::from_secs(config.jwt_refresh_token_duration_seconds),
    )
    .await?;

    // Initialize Email Service
    let email_service = service::EmailService::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any); // TODO: Restrict later
    let state = AppState::new(conn, config, session_manager, email_service);

    Ok(ApiRouter::new()
        .merge(routes::get_router(state))
        .layer(cors))
}
