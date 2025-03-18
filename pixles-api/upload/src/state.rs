use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::{config::UploadServerConfig, metadata::FileDatabase};

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub config: UploadServerConfig,
    pub file_db: FileDatabase,
}
