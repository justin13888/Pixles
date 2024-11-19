use async_graphql::{Error, ErrorExtensions};
use axum::http::HeaderMap;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Define your claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    // TODO: Add more details
}

#[derive(Clone)]
pub struct UserContext {
    user_id: String,
}

// JWT validation error
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Missing token")]
    TokenMissing,
    #[error("Invalid token")]
    TokenInvalid,
    #[error("Expired token")]
    TokenExpired,
}

impl ErrorExtensions for AuthError {
    // lets define our base extensions
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|err, e| match self {
            AuthError::TokenExpired => e.set("code", "TOKEN_MISSING"),
            AuthError::TokenInvalid => e.set("code", "TOKEN_INVALID"),
            AuthError::TokenMissing => e.set("code", "TOKEN_EXPIRED"),
        })
    }
}

fn get_token_from_headers(headers: &HeaderMap) -> Result<String, AuthError> {
    let auth_header = headers
        .get("Authorization")
        .ok_or(AuthError::TokenMissing)?
        .to_str()
        .map_err(|_| AuthError::TokenInvalid)?;

    // Check if it's a Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::TokenInvalid);
    }

    // Extract the token part
    Ok(auth_header[7..].to_string())
}

fn validate_token(token: &str) -> Result<Claims, AuthError> {
    // Your JWT secret key - should be loaded from environment variables
    const JWT_SECRET: &[u8] = b"your-secret-key";

    // Decode and validate the token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| AuthError::TokenInvalid)?;

    Ok(token_data.claims)
}

pub fn get_user_context_from_headers(headers: &HeaderMap) -> Result<UserContext, AuthError> {
    let token = get_token_from_headers(headers)?;
    let claims = validate_token(&token)?;

    Ok(UserContext {
        user_id: claims.sub,
    })
}
