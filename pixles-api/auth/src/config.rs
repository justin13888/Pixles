use environment::ServerConfig;
use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Clone)]
pub struct AuthConfig {
    /// Server host (e.g. "0.0.0.0")
    pub host: String,
    /// Server port (e.g. 3000)
    pub port: u16,
    /// Public domain (e.g. "api.pixles.com")
    pub domain: String,

    /// EdDSA encoding key
    pub jwt_eddsa_encoding_key: EncodingKey,
    /// EdDSA decoding key
    pub jwt_eddsa_decoding_key: DecodingKey,

    /// JWT refresh token duration in seconds
    pub jwt_refresh_token_duration_seconds: usize,
    /// JWT access token duration in seconds
    pub jwt_access_token_duration_seconds: usize,
}

impl From<&ServerConfig> for AuthConfig {
    fn from(config: &ServerConfig) -> Self {
        Self {
            host: config.host.clone(),
            port: config.port,
            domain: config.domain.clone(),
            jwt_eddsa_encoding_key: (*config.jwt_eddsa_encoding_key).clone(),
            jwt_eddsa_decoding_key: (*config.jwt_eddsa_decoding_key).clone(),
            jwt_refresh_token_duration_seconds: config.jwt_refresh_token_duration_seconds,
            jwt_access_token_duration_seconds: config.jwt_access_token_duration_seconds,
        }
    }
}
