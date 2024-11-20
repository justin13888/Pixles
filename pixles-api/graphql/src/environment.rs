use dotenvy::dotenv;
use serde::Deserialize;
use std::{env, num::ParseIntError};
use thiserror::Error;
use tracing::level_filters::LevelFilter;

#[derive(Debug, Error)]
pub enum EnvironmentError {
    #[error("Environment variable not found: {0}")]
    MissingVariable(String),
    #[error("Failed to parse environment variable: {0}")]
    ParseError(String),
}

#[derive(Debug, Deserialize)]
pub struct EnvironmentDatabase {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct EnvironmentServer {
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct Environment {
    pub database: EnvironmentDatabase,
    pub server: EnvironmentServer,
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
                v.parse()
                    .map_err(|e: ParseIntError| EnvironmentError::ParseError(e.to_string()))
            })
        };

        Ok(Self {
            database: EnvironmentDatabase {
                url: load_env("DATABASE_URL")?,
            },
            server: EnvironmentServer {
                host: load_env("SERVER_HOST").unwrap_or("0.0.0.0".to_string()),
                port: load_env_int("SERVER_PORT").unwrap_or(3000),
            },
            log_level: match load_env("LOG_LEVEL") {
                Ok(level) => level
                    .parse::<LevelFilter>()
                    .map_err(|e| EnvironmentError::ParseError(e.to_string()))?,
                Err(_) => {
                    if cfg!(debug_assertions) {
                        LevelFilter::DEBUG
                    } else {
                        LevelFilter::INFO
                    }
                }
            },
        })
    }
}
