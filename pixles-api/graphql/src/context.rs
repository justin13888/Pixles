use std::{collections::HashSet, sync::Arc};

use crate::{
    auth::{Claims, UserRole},
    config::GraphqlServerConfig,
};
use async_graphql::{Error, ErrorExtensions, ServerError};
use axum::http::HeaderMap;
use jsonwebtoken::TokenData;
use sea_orm::DatabaseConnection;
use secrecy::{ExposeSecret, SecretString};
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum UserType {
    /// A normal user (user_id)
    User(String),
    /// An admin user (user_id)
    Admin(String),
    /// A guest user
    Guest,
}

#[derive(Debug, Clone)]
pub struct UserContext {
    user_type: UserType,
    scopes: HashSet<String>,
}

impl UserContext {
    pub fn from_headers(
        headers: &HeaderMap,
        config: &GraphqlServerConfig,
    ) -> Result<Self, AuthError> {
        let mut scopes = None;
        let user_type: UserType = match get_token_from_headers(headers) {
            Ok(token) => {
                let TokenData { claims, .. } =
                    Claims::decode(token.expose_secret(), &config.jwt_eddsa_decoding_key)?;

                scopes = Some(claims.scopes);

                match claims.role {
                    UserRole::User => UserType::User(claims.sub),
                    UserRole::Admin => UserType::Admin(claims.sub),
                }
            }
            Err(AuthError::TokenMissing) => UserType::Guest,
            Err(e) => return Err(e),
        };

        Ok(Self {
            user_type,
            scopes: scopes.unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DbContext {
    pub conn: Arc<DatabaseConnection>,
}

#[derive(Debug, Clone)]
pub struct AppContext {
    pub user: UserContext,
    pub db: DbContext,
}

// JWT validation error
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Missing token")]
    TokenMissing,
    #[error("Invalid token: {0}")]
    TokenInvalid(#[from] jsonwebtoken::errors::Error),
    #[error("Expired token")]
    TokenExpired,
    #[error("Unexpected authorization header format")]
    UnexpectedHeaderFormat,
}

impl ErrorExtensions for AuthError {
    // lets define our base extensions
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|err, e| match self {
            AuthError::TokenExpired => e.set("code", "TOKEN_MISSING"),
            AuthError::TokenInvalid(msg) => e.set("code", format!("TOKEN_INVALID ({})", msg)),
            AuthError::TokenMissing => e.set("code", "TOKEN_EXPIRED"),
            AuthError::UnexpectedHeaderFormat => e.set("code", "UNEXPECTED_HEADER_FORMAT"),
        })
    }
}

impl From<AuthError> for ServerError {
    fn from(err: AuthError) -> Self {
        let field_error = err.extend();
        ServerError::new(field_error.message.as_str(), None)
    }
}

/// Get the token from the Authorization header
fn get_token_from_headers(headers: &HeaderMap) -> Result<SecretString, AuthError> {
    let auth_header = headers
        .get("Authorization")
        .ok_or(AuthError::TokenMissing)?
        .to_str()
        .map_err(|_| AuthError::UnexpectedHeaderFormat)?;

    // Check if it's a Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::UnexpectedHeaderFormat);
    }

    // Extract the token part
    Ok(SecretString::from(&auth_header[7..]))
}

// TODO: Bench all the functions here (used in each context)
