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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claims::Claims;
    use base64::Engine;
    use jsonwebtoken::DecodingKey;
    use ring::signature::Ed25519KeyPair;
    use ring::signature::KeyPair;

    fn get_test_keys() -> (EncodingKey, DecodingKey) {
        let doc: Vec<u8> = base64::engine::general_purpose::STANDARD
            .decode("MC4CAQAwBQYDK2VwBCIEIG73KilXg8qazIq8mNGzuPEHYPLY3WXR1uOS7ZxNkefV")
            .unwrap();
        let pair = Ed25519KeyPair::from_pkcs8_maybe_unchecked(&doc).unwrap();
        let encoding_key = EncodingKey::from_ed_der(&doc);
        let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

        (encoding_key, decoding_key)
    }

    #[test]
    fn test_create_access_token() {
        let (encoding_key, decoding_key) = get_test_keys();
        let user_id = "user123";
        let scopes = vec![Scope::ReadUser];

        let (token, exp) =
            TokenService::create_access_token(user_id, scopes.clone(), &encoding_key).unwrap();

        let decoded = Claims::decode(&token, &decoding_key).unwrap();
        assert_eq!(decoded.claims.sub, user_id);
        assert_eq!(decoded.claims.scopes, scopes);
        assert_eq!(decoded.claims.exp, exp);
        assert!(decoded.claims.sid.is_none());
    }

    #[test]
    fn test_create_refresh_token() {
        let (encoding_key, decoding_key) = get_test_keys();
        let user_id = "user123";
        let sid = "session123";

        let token =
            TokenService::create_refresh_token(user_id, sid.to_string(), &encoding_key).unwrap();

        let decoded = Claims::decode(&token, &decoding_key).unwrap();
        assert_eq!(decoded.claims.sub, user_id);
        assert_eq!(decoded.claims.sid, Some(sid.to_string()));
        assert_eq!(decoded.claims.scopes, vec![Scope::RefreshToken]);
    }
}
