use axum::extract::DefaultBodyLimit;
use config::UploadServerConfig;
use eyre::Result;
use metadata::FileDatabase;
use sea_orm::DatabaseConnection;

use aide::axum::ApiRouter;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::{config::validate_config, state::AppState};

mod config;
mod error;
mod metadata;
mod routes;
mod state;

pub async fn get_router<C: Into<UploadServerConfig>>(
    conn: DatabaseConnection,
    config: C,
) -> Result<ApiRouter> {
    let config = config.into();
    let config_warnings = validate_config(&config).map_err(|e| {
        eyre::eyre!(
            "Upload server configuration is invalid: {}. Please fix the configuration and try again.",
            e
        )
    })?;
    if !config_warnings.is_empty() {
        info!("Upload server config warnings: {:?}", config_warnings);
    }

    // Initialize database
    let file_db = FileDatabase::new(config.clone()).await?;
    // TODO: Read existing upload folder and db to ensure old stuff are recovered. Need to decide whether to attempt to recover whatever (e.g. intermittent, brief outages) or just restart everything

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let default_body_limit = DefaultBodyLimit::max(config.max_file_size);
    let state = AppState {
        conn,
        config,
        file_db,
    };

    Ok(ApiRouter::new()
        .merge(routes::get_router(state))
        .layer(cors)
        .layer(default_body_limit))
}
