use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::AuthConfig;
use crate::session::SessionManager;

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub config: AuthConfig,
    pub session_manager: SessionManager,
    pub email_service: crate::service::EmailService,
}
