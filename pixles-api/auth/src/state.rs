use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::AuthConfig;

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub config: AuthConfig,
}
