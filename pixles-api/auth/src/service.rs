use crate::claims::{Claims, Scope};
use crate::config::AuthConfig;
use crate::errors::ClaimValidationError;

pub struct AuthService {
    config: AuthConfig,
}

impl AuthService {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
        // TODO: I shouldn't need entire authconfig because it only needs to
        // decode keys
    }

    /// Get [Claims] from token string
    pub fn get_claims(&self, token: &str) -> Result<Claims, ClaimValidationError> {
        let claims = Claims::decode(token, &self.config.jwt_eddsa_decoding_key)?;
        Ok(claims.claims)
    }

    /// Validate [Claims] by scopes
    pub fn validate_claims(
        &self,
        claims: &Claims,
        required_scopes: &[Scope],
    ) -> Result<(), ClaimValidationError> {
        if !claims.has_scopes(required_scopes) {
            return Err(ClaimValidationError::InvalidScopes);
        }
        Ok(())
    }
}

// TODO: Replace with real email service (e.g. AWS SES, SendGrid, SMTP)
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
