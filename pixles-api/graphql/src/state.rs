use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::schema::AppSchema;
use environment::ServerConfig;

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub conn: Arc<DatabaseConnection>,
    pub config: ServerConfig,
}
