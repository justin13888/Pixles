use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::AuthConfig;
use crate::service::{AuthService, EmailService, PasswordService};
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
    pub auth_service: AuthService,
    pub password_service: PasswordService,
}

impl AppState {
    pub fn new(
        conn: DatabaseConnection,
        config: AuthConfig,
        session_manager: SessionManager,
        email_service: crate::service::EmailService,
    ) -> Self {
        let auth_service = AuthService::new(config.clone());
        let password_service = PasswordService::new(1000); // 1s minimum

        Self {
            inner: Arc::new(AppStateInner {
                conn,
                config,
                session_manager,
                email_service,
                auth_service,
                password_service,
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
