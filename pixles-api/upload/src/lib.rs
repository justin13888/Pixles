use axum::{extract::DefaultBodyLimit, routing::post, Router};
use config::UploadServerConfig;
use eyre::{eyre, Result};
use metadata::FileDatabase;
use routes::upload_file;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing::info;

use crate::state::AppState;

mod config;
mod error;
mod metadata;
mod routes;
mod state;

/// Validate the configuration
/// Returns a list of warnings
pub fn validate_config(config: &UploadServerConfig) -> Result<Vec<String>> {
    let mut warnings = vec![];
    if config.max_file_size >= config.max_cache_size {
        return Err(eyre!("max_file_size must be less than max_cache_size"));
    }

    // Warn max_file_size allows < 10 concurrent files
    if config.max_cache_size / config.max_file_size < 10 {
        warnings.push(
            "Based on current max_cache_size, max_file_size allows < 10 concurrent files"
                .to_string(),
        );
    }

    // Warn if upload_dir is a non-empty directory
    if config.upload_dir.is_dir() && config.upload_dir.read_dir()?.count() > 0 {
        warnings.push("upload_dir is non-empty. This may be from server restarts.".to_string());
    }

    // Warn if sled_db_dir is an existing directory
    if config.sled_db_dir.is_dir() {
        warnings.push(
            "sled_db_dir is an existing directory. This may be from server restarts.".to_string(),
        );
    }

    Ok(warnings)
}

pub async fn get_router<C: Into<UploadServerConfig>>(
    conn: Arc<DatabaseConnection>,
    config: C,
) -> Result<Router> {
    let config = config.into();
    let config_warnings = validate_config(&config)?;
    if !config_warnings.is_empty() {
        info!("Upload server config warnings: {:?}", config_warnings);
    }

    // Initialize database
    let file_db = FileDatabase::new(config.clone()).await?;
    // TODO: Read existing upload folder and db to ensure old stuff are recovered. Need to decide whether to attempt to recover whatever (e.g. intermittent, brief outages) or just restart everything

    let default_body_limit = DefaultBodyLimit::max(config.max_file_size);
    let state = AppState {
        conn,
        config,
        file_db,
    };

    let router = Router::new()
        .route("/upload", post(upload_file))
        .layer(default_body_limit) // TODO: Ensure limit is respected
        .with_state(Arc::new(state));

    // TODO: Complete implementation

    Ok(router)
}
