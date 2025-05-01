use std::sync::Arc;

use config::AuthConfig;
use eyre::Result;
use sea_orm::DatabaseConnection;
use state::AppState;
use tower_http::cors::{Any, CorsLayer};
use utoipa_axum::router::OpenApiRouter;

pub mod claims;
pub mod config;
pub mod constants;
pub mod error;
#[cfg(feature = "server")]
mod models;
pub mod oidc;
pub mod roles;
#[cfg(feature = "server")]
mod routes;
pub mod service;
#[cfg(feature = "server")]
mod state;
pub mod utils;

#[cfg(feature = "server")]
pub async fn get_router<C: Into<AuthConfig>>(
    conn: Arc<DatabaseConnection>,
    config: C,
) -> Result<OpenApiRouter> {
    let config = config.into();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any); // TODO: Restrict later
    let state = AppState { conn, config };

    Ok(OpenApiRouter::new()
        .nest("/auth", routes::get_router(state))
        .layer(cors))
}
