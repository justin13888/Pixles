use config::AuthConfig;
use eyre::Result;
use sea_orm::DatabaseConnection;
use state::AppState;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

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
#[derive(OpenApi)]
#[openapi(components(
    responses(
        models::responses::TokenResponse,
        models::responses::ValidateTokenResponse
    ),
    schemas(
        errors::AuthError,
        models::UserProfile,
        models::errors::BadRegisterUserRequestError,
        models::responses::TokenResponse,
        models::responses::ValidateTokenResponse
    )
))]
pub struct AuthApiDoc;

#[cfg(feature = "server")]
pub async fn get_router<C: Into<AuthConfig>>(
    conn: DatabaseConnection,
    config: C,
) -> Result<OpenApiRouter> {
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

    Ok(OpenApiRouter::with_openapi(AuthApiDoc::openapi())
        .merge(routes::get_router(state))
        .layer(cors))
}
