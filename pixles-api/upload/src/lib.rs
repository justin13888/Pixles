use axum::Router;
use config::UploadServerConfig;
use eyre::{eyre, Result};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing::info;

use crate::state::AppState;

mod config;
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

    // Warn if db_path is an existing file
    if config.db_path.is_file() {
        warnings.push("db_path is an existing file. This may be from server restarts.".to_string());
    }

    Ok(warnings)
}

pub async fn get_router(
    conn: Arc<DatabaseConnection>,
    config: UploadServerConfig,
) -> Result<Router> {
    let config_warnings = validate_config(&config)?;
    if !config_warnings.is_empty() {
        info!("Config warnings: {:?}", config_warnings);
    }

    // TODO: Add route to expose these configs

    let state = AppState { conn, config };

    let router = Router::new().with_state(state);

    // TODO: Complete implementation

    Ok(router)
}
