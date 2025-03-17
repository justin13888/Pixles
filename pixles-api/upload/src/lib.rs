use axum::Router;
use config::UploadServerConfig;
use eyre::Result;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::state::AppState;

mod config;
mod state;

pub async fn get_router(
    conn: Arc<DatabaseConnection>,
    config: UploadServerConfig,
) -> Result<Router> {
    let state = AppState { conn, config };

    let router = Router::new().with_state(state);

    // TODO: Complete implementation

    Ok(router)
}
