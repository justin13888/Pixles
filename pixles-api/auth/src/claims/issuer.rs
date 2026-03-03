/// Returns the JWT token issuer. Reads from PIXLES_ISSUER env var if set.
pub fn get_auth_issuer() -> String {
    std::env::var("PIXLES_ISSUER").unwrap_or_else(|_| crate::constants::ISSUER.to_string())
}
