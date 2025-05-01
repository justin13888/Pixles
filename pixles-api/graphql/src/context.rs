use std::{collections::HashSet, sync::Arc};

use async_graphql::{Error, ErrorExtensions, ServerError};
use auth::{error::JWTValidationError, service::AuthService, claims::Claims, roles::UserRole};
use axum::http::HeaderMap;
use sea_orm::DatabaseConnection;
use secrecy::{ExposeSecret, SecretString};

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
        auth_service: &AuthService,
    ) -> Result<Self, UserContextError> {
        let mut scopes = None;
        let user_type: UserType = match get_token_from_headers(headers) {
            Ok(token) => {
                let claims = auth_service.get_claims(&token.expose_secret())?;

                scopes = Some(claims.scopes);

                match claims.role {
                    UserRole::User => UserType::User(claims.sub),
                    UserRole::Admin => UserType::Admin(claims.sub),
                }
            }
            Err(JWTValidationError::TokenMissing) => UserType::Guest,
            Err(e) => return Err(e.into()),
        };

        Ok(Self {
            user_type,
            scopes: scopes.map_or_else(HashSet::new, |scopes| scopes.into_iter().collect()),
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

pub struct UserContextError(JWTValidationError);

impl From<JWTValidationError> for UserContextError {
    fn from(err: JWTValidationError) -> Self {
        Self(err)
    }
}

impl ErrorExtensions for UserContextError {
    // lets define our base extensions
    fn extend(&self) -> Error {
        let error = &self.0;
        Error::new(format!("{}", error)).extend_with(|err, e| match error {
            JWTValidationError::TokenExpired => e.set("code", "TOKEN_MISSING"),
            JWTValidationError::TokenInvalid(msg) => e.set("code", format!("TOKEN_INVALID ({})", msg)),
            JWTValidationError::TokenMissing => e.set("code", "TOKEN_EXPIRED"),
            JWTValidationError::UnexpectedHeaderFormat => e.set("code", "UNEXPECTED_HEADER_FORMAT"),
        })
    }
}

impl From<UserContextError> for ServerError {
    fn from(err: UserContextError) -> Self {
        let field_error = err.extend();
        ServerError::new(field_error.message.as_str(), None)
    }
}

/// Get the token from the Authorization header
fn get_token_from_headers(headers: &HeaderMap) -> Result<SecretString, JWTValidationError> {
    let auth_header = headers
        .get("Authorization")
        .ok_or(JWTValidationError::TokenMissing)?
        .to_str()
        .map_err(|_| JWTValidationError::UnexpectedHeaderFormat)?;

    // Check if it's a Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(JWTValidationError::UnexpectedHeaderFormat);
    }

    // Extract the token part
    Ok(SecretString::from(&auth_header[7..]))
}

// TODO: Bench all the functions here (used in each context)
