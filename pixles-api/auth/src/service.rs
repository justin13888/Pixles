use crate::{claims::Claims, config::AuthConfig, error::JWTValidationError};

pub struct AuthService {
    config: AuthConfig,
}

impl AuthService {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    pub fn get_claims(&self, token: &str) -> Result<Claims, JWTValidationError> {
        let claims = Claims::decode(token, &self.config.jwt_eddsa_decoding_key)?;
        Ok(claims.claims)
    }
}
