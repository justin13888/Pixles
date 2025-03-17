use dotenvy::dotenv;
use std::{env, num::ParseIntError};
use thiserror::Error;
use tracing::level_filters::LevelFilter;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use jsonwebtoken::{DecodingKey, EncodingKey};
use ring::signature::Ed25519KeyPair;

use crate::jwt::convert_ed25519_der_to_jwt_keys;

mod jwt;

#[derive(Debug, Error)]
pub enum EnvironmentError {
    /// MissingVariable(key)
    #[error("Environment variable not found: \"{0}\"")]
    MissingVariable(String),
    /// ParseError(key, error)
    #[error("Failed to parse environment variable \"{0}\": {1}")]
    ParseError(String, String),
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone)]
pub struct ServerConfig {
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

impl std::fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("domain", &self.domain)
            .field("jwt_eddsa_encoding_key", &"REDACTED")
            .field("jwt_eddsa_decoding_key", &"REDACTED")
            .field(
                "jwt_refresh_token_duration_seconds",
                &self.jwt_refresh_token_duration_seconds,
            )
            .field(
                "jwt_access_token_duration_seconds",
                &self.jwt_access_token_duration_seconds,
            )
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub log_level: LevelFilter,
}

impl Environment {
    pub fn load() -> Result<Self, EnvironmentError> {
        dotenv().ok();

        let load_env = |key: &str| {
            env::var(key).map_err(|_| EnvironmentError::MissingVariable(key.to_string()))
        };
        let load_env_int = |key: &str| {
            load_env(key).and_then(|v| {
                v.parse().map_err(|e: ParseIntError| {
                    EnvironmentError::ParseError(key.to_string(), e.to_string())
                })
            })
        };
        let load_env_usize = |key: &str| {
            load_env(key).and_then(|v| {
                v.parse().map_err(|e: ParseIntError| {
                    EnvironmentError::ParseError(key.to_string(), e.to_string())
                })
            })
        };

        let load_jwt_ed25519_keys = |key: &str| {
            load_env(key).and_then(|s| {
                let der = BASE64.decode(s).map_err(|e| {
                    EnvironmentError::ParseError(
                        key.to_string(),
                        format!("Unable to decode base64: {}", e),
                    )
                })?;

                convert_ed25519_der_to_jwt_keys(&der).map_err(|e| {
                    EnvironmentError::ParseError(
                        key.to_string(),
                        format!("Unable to convert DER to JWT keys: {}", e),
                    )
                })
            })
        };

        let (jwt_eddsa_encoding_key, jwt_eddsa_decoding_key) =
            load_jwt_ed25519_keys("JWT_ED25519_DER")?;

        let load_log_level = |key: &str| {
            load_env(key).and_then(|s| {
                s.parse::<LevelFilter>()
                    .map_err(|e| EnvironmentError::ParseError(key.to_string(), e.to_string()))
            })
        };

        Ok(Self {
            database: DatabaseConfig {
                url: load_env("DATABASE_URL")?,
            },
            server: ServerConfig {
                host: load_env("SERVER_HOST").unwrap_or("0.0.0.0".to_string()),
                port: load_env_int("SERVER_PORT").unwrap_or(3000),
                domain: load_env("SERVER_DOMAIN").unwrap_or("localhost".to_string()),
                jwt_eddsa_encoding_key,
                jwt_eddsa_decoding_key,
                jwt_refresh_token_duration_seconds: load_env_usize(
                    "JWT_REFRESH_TOKEN_DURATION_SECONDS",
                )
                .unwrap_or(60 * 60 * 24 * 30), // 30 days
                jwt_access_token_duration_seconds: load_env_usize(
                    "JWT_ACCESS_TOKEN_DURATION_SECONDS",
                )
                .unwrap_or(60 * 10), // 10 minutes
            },
            log_level: load_log_level("LOG_LEVEL").unwrap_or(if cfg!(debug_assertions) {
                LevelFilter::TRACE
            } else {
                LevelFilter::INFO
            }),
        })
    }
}
