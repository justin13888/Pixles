use dotenvy::dotenv;
use serde::Deserialize;
use std::{env, num::ParseIntError};
use thiserror::Error;

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

#[derive(Debug, Deserialize)]
pub struct Environment {
    pub database: EnvironmentDatabase,
    pub server: EnvironmentServer,
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
                host: load_env("SERVER_HOST")?,
                port: load_env_int("SERVER_PORT")?,
            },
        })
    }
}
