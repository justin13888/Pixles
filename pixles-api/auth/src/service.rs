use crate::{
    claims::{Claims, Scope},
    config::AuthConfig,
    errors::ClaimValidationError,
};

pub struct AuthService {
    config: AuthConfig,
}

impl AuthService {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
        // TODO: I shouldn't need entire authconfig because it only needs to decode keys
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
