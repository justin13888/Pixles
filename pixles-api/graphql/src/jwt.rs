use chrono::Utc;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    /// A normal user
    User,
    /// An admin user
    Admin,
}

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
    pub scopes: HashSet<String>, // TODO: Work out specifics of what scopes are needed
}

impl Claims {
    /// Creates a standard Claims
    pub fn new(user_id: String, user_role: UserRole, scopes: HashSet<String>) -> Self {
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
        self.scopes.contains(scope)
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

/// Convert a PKCS#8 v1 DER-encoded ED25519 key to corresponding jsonwebtoken EdDSA keys
pub fn convert_ed25519_der_to_jwt_keys(
    der: &[u8],
) -> Result<(EncodingKey, DecodingKey), ring::error::KeyRejected> {
    let pair = Ed25519KeyPair::from_pkcs8_maybe_unchecked(der)?;

    Ok((
        EncodingKey::from_ed_der(der),
        DecodingKey::from_ed_der(pair.public_key().as_ref()),
    ))
}
// TODO: Test ^

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

    use super::*;

    /// Test we parse a DER-encoded ED25519 keypair
    #[test]
    fn test_generate_keypair() {
        let doc = BASE64
            .decode("MC4CAQAwBQYDK2VwBCIEIG73KilXg8qazIq8mNGzuPEHYPLY3WXR1uOS7ZxNkefV")
            .unwrap();
        assert!(convert_ed25519_der_to_jwt_keys(doc.as_ref()).is_ok());
    }

    /// Test we fail to parse a bad DER-encoded ED25519 keypair
    #[test]
    fn test_generate_keypair_bad_der() {
        let doc = BASE64.decode("Ym9ndXMK").unwrap(); // "bogus" in base64
        assert!(convert_ed25519_der_to_jwt_keys(doc.as_ref()).is_err());
    }

    /// Test we encode and decode a token correctly
    #[test]
    fn test_encode_decode() {
        // Generate test keypair
        let doc = BASE64
            .decode("MC4CAQAwBQYDK2VwBCIEIG73KilXg8qazIq8mNGzuPEHYPLY3WXR1uOS7ZxNkefV")
            .unwrap();
        let (encoding_key, decoding_key) = convert_ed25519_der_to_jwt_keys(doc.as_ref()).unwrap();

        let claims = Claims::new("user_id".to_string(), HashSet::new());

        let token = claims.encode(&encoding_key).unwrap();
        let decoded = Claims::decode(&token, &decoding_key).unwrap();
        assert_eq!(decoded.header.alg, Algorithm::EdDSA);
        assert_eq!(decoded.claims, claims);
    }
}

// TODO: Bench core functions (being used many times in context.rs)
