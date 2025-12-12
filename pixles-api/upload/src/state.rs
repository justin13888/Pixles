use sea_orm::DatabaseConnection;

use crate::{config::UploadServerConfig, metadata::FileDatabase};

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub config: UploadServerConfig,
    pub file_db: FileDatabase,
}
