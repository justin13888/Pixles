use crate::claims::{Claims, Scope};
use crate::errors::AuthError;
use jsonwebtoken::EncodingKey;

pub struct TokenService;

impl TokenService {
    pub fn create_access_token(
        user_id: &str,
        scopes: Vec<Scope>,
        encoding_key: &EncodingKey,
    ) -> Result<(String, u64), AuthError> {
        let claims = Claims::new_access_token(user_id.to_string(), scopes);
        let token = claims
            .encode(encoding_key)
            .map_err(|e| AuthError::InternalServerError(e.into()))?;
        Ok((token, claims.exp))
    }

    pub fn create_refresh_token(
        user_id: &str,
        sid: String,
        encoding_key: &EncodingKey,
    ) -> Result<String, AuthError> {
        let claims = Claims::new_refresh_token(user_id.to_string(), sid);
        claims
            .encode(encoding_key)
            .map_err(|e| AuthError::InternalServerError(e.into()))
    }
}
