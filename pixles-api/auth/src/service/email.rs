#[derive(Clone)]
pub struct EmailService;

impl EmailService {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_password_reset_email(
        &self,
        email: &str,
        token: &str,
    ) -> Result<(), eyre::Report> {
        // Mock implementation
        tracing::info!(
            "Mock Email Service: Sending password reset email to {}. Token: {}",
            email,
            token
        );
        // In real implementation, generate a link like https://pixles.com/reset-password?token=...
        Ok(())
    }
}

impl Default for EmailService {
    fn default() -> Self {
        Self::new()
    }
}
