use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::AuthConfig;
use crate::service::EmailService;
use crate::session::SessionManager;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub conn: DatabaseConnection,
    pub config: AuthConfig,
    pub session_manager: SessionManager,
    pub email_service: EmailService,
}

impl AppState {
    pub fn new(
        conn: DatabaseConnection,
        config: AuthConfig,
        session_manager: SessionManager,
        email_service: crate::service::EmailService,
    ) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                conn,
                config,
                session_manager,
                email_service,
            }),
        }
    }
}

impl std::ops::Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
