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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;
    use secrecy::ExposeSecret;

    #[test]
    fn test_get_token_valid() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_static("Bearer my_token"));

        let token = get_token_from_headers(&headers).unwrap();
        assert_eq!(token.expose_secret(), "my_token");
    }

    #[test]
    fn test_get_token_missing_header() {
        let headers = HeaderMap::new();
        let result = get_token_from_headers(&headers);
        assert!(matches!(result, Err(ClaimValidationError::TokenMissing)));
    }

    #[test]
    fn test_get_token_invalid_format() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("Basic base64code"),
        );

        let result = get_token_from_headers(&headers);
        assert!(matches!(
            result,
            Err(ClaimValidationError::UnexpectedHeaderFormat)
        ));
    }

    #[test]
    fn test_get_token_malformed() {
        let mut headers = HeaderMap::new();
        // Just "Bearer" w/o space or token
        headers.insert("Authorization", HeaderValue::from_static("Bearer"));
        let result = get_token_from_headers(&headers);
        // "Bearer".len() is 6. [7..] would panic if we didn't check starts_with("Bearer ") which includes space (len 7)
        // CODE CHECK:
        // if !auth_header.starts_with("Bearer ")

        // "Bearer" (len 6) -> starts_with("Bearer ") is false.
        assert!(matches!(
            result,
            Err(ClaimValidationError::UnexpectedHeaderFormat)
        ));
    }
}
