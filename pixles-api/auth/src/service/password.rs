use super::email::EmailService;
use crate::errors::AuthError;
use chrono::{Duration, Utc};
use sea_orm::DatabaseConnection;
use service::user as UserService;
use std::time::Instant;

pub struct PasswordService {
    min_operation_time: std::time::Duration,
}

impl PasswordService {
    pub fn new(min_operation_time_ms: u64) -> Self {
        Self {
            min_operation_time: std::time::Duration::from_millis(min_operation_time_ms),
        }
    }

    pub async fn request_reset(
        &self,
        conn: &DatabaseConnection,
        email_service: &EmailService,
        email: &str,
    ) -> Result<(), AuthError> {
        let start = Instant::now();

        // Find user by email
        let user_result = UserService::Query::find_user_by_email(conn, email).await;

        let result = match user_result {
            Ok(Some(user)) => {
                // Generate token
                let token = nanoid::nanoid!();
                let expires_at = Utc::now() + Duration::hours(1);

                // Update user with token
                match UserService::Mutation::update_password_reset_token(
                    conn,
                    user.id.clone(),
                    token.clone(),
                    expires_at,
                )
                .await
                {
                    Ok(_) => {
                        // Send email
                        email_service
                            .send_password_reset_email(&user.email, &token)
                            .await
                            .map_err(|e| AuthError::InternalServerError(e))
                    }
                    Err(e) => Err(AuthError::InternalServerError(e.into())),
                }
            }
            Ok(None) => {
                // User not found - pretend success
                Ok(())
            }
            Err(e) => Err(AuthError::InternalServerError(e.into())),
        };

        // Ensure minimum time elapsed
        let elapsed = start.elapsed();
        if elapsed < self.min_operation_time {
            tokio::time::sleep(self.min_operation_time - elapsed).await;
        }

        result
    }
}
