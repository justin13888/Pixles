use chrono::Utc;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use crate::roles::UserRole;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    // Standard JWT claims
    /// Subject - the user ID
    pub sub: String,
    /// Expiration time (as UTC timestamp)
    pub exp: usize,
    /// Issued at (as UTC timestamp)
    pub iat: usize,
    /// JWT ID - Unique identifier for this token
    pub jti: String,
    /// Issuer (e.g. "api.pixles.com")
    pub iss: String,

    // Custom claims
    /// User type
    pub role: UserRole,
    /// Permissions/scopes granted to this token
    pub scopes: Vec<String>, // TODO: Work out specifics of what scopes are needed
}

impl Claims {
    /// Creates a standard Claims
    pub fn new<S: Into<Vec<String>>>(user_id: String, user_role: UserRole, scopes: S) -> Self {
        let scopes = scopes.into();
        let iat = Utc::now().timestamp() as usize;
        let exp = iat + 3600; // 1 hour

        Self {
            sub: user_id,
            exp,
            iat,
            jti: uuid::Uuid::new_v4().to_string(),
            iss: "pixles-api".to_string(), // TODO: might want it to be more unique to support federation

            role: user_role,
            scopes,
        }
    }

    /// Returns true if the token is still valid
    pub fn is_valid(&self) -> bool {
        self.exp > Utc::now().timestamp() as usize
    }

    /// Checks if a specific scope is present
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }

    /// Decode from a token string
    /// Assumes `key` uses EdDSA
    pub fn decode(
        token: &str,
        key: &DecodingKey,
    ) -> Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        decode::<Self>(token, key, &Validation::new(Algorithm::EdDSA))
    }

    /// Encode to a token string
    /// Assumes `key` uses EdDSA
    pub fn encode(&self, key: &EncodingKey) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&jsonwebtoken::Header::new(Algorithm::EdDSA), &self, key)
    }
}
// TODO: Test ^^

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
    use ring::signature::{Ed25519KeyPair, KeyPair};

    use super::*;

    /// Test we encode and decode a token correctly
    #[test]
    fn test_encode_decode() {
        // Generate test keypair
        let doc: Vec<u8> = BASE64
            .decode("MC4CAQAwBQYDK2VwBCIEIG73KilXg8qazIq8mNGzuPEHYPLY3WXR1uOS7ZxNkefV")
            .unwrap();
        let pair = Ed25519KeyPair::from_pkcs8_maybe_unchecked(&doc).unwrap();
        let encoding_key = EncodingKey::from_ed_der(&doc);
        let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

        // Create claims
        let claims = Claims::new("user_id".to_string(), UserRole::User, vec![]);

        // Encode and decode
        let token = claims.encode(&encoding_key).unwrap();
        let decoded = Claims::decode(&token, &decoding_key).unwrap();

        // Check header and claims
        assert_eq!(decoded.header.alg, Algorithm::EdDSA);
        assert_eq!(decoded.claims, claims);
    }
}

// TODO: Bench core functions (being used many times for authorization)
