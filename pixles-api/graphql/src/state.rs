use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{environment::ServerConfig, schema::AppSchema};

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub conn: Arc<DatabaseConnection>,
    pub config: ServerConfig,
}
