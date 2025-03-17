use sea_orm::DatabaseConnection;
use std::sync::Arc;

use environment::ServerConfig;

use crate::config::UploadServerConfig;

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub config: UploadServerConfig,
}
