use config::AuthConfig;
use eyre::Result;
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::prelude::*;
use sea_orm::DatabaseConnection;
use state::AppState;

pub mod claims;
pub mod config;
pub mod constants;
pub mod errors;
#[cfg(feature = "server")]
pub mod models;
pub mod oidc;
pub mod roles;
#[cfg(feature = "server")]
mod routes;
pub mod service;
#[cfg(feature = "server")]
pub mod session;
#[cfg(feature = "server")]
pub mod state;
pub mod utils;
#[cfg(feature = "server")]
pub mod validation;

#[cfg(feature = "server")]
pub async fn get_router<C: Into<AuthConfig>>(
    conn: DatabaseConnection,
    config: C,
) -> Result<Router> {
    let config = config.into();

    let session_manager = session::SessionManager::new(
        config.valkey_url.clone(),
        std::time::Duration::from_secs(config.jwt_refresh_token_duration_seconds),
    )
    .await?;

    // Initialize Email Service
    let email_service = service::EmailService::new();

    // CORS configuration - TODO: Restrict later
    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers("*")
        .into_handler();

    let state = AppState::new(conn, config, session_manager, email_service);

    let router = routes::get_router(state);

    // Wrap with CORS
    let router = Router::new().hoop(cors).push(router);

    Ok(router)
}
