use axum::http::HeaderMap;
use secrecy::SecretString;

use crate::errors::ClaimValidationError;

/// Get the token from the Authorization header
pub fn get_token_from_headers(headers: &HeaderMap) -> Result<SecretString, ClaimValidationError> {
    let auth_header = headers
        .get("Authorization")
        .ok_or(ClaimValidationError::TokenMissing)?
        .to_str()
        .map_err(|_| ClaimValidationError::UnexpectedHeaderFormat)?;

    // Check if it's a Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(ClaimValidationError::UnexpectedHeaderFormat);
    }

    // Extract the token part
    Ok(SecretString::from(&auth_header[7..]))
}
