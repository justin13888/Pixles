use argon2::{
    password_hash::{
        rand_core::OsRng, Error as PasswordHashError, PasswordHash, PasswordHasher,
        PasswordVerifier, SaltString,
    },
    Argon2,
};

// TODO: use these functions vv

/// Hash a password using Argon2id v19 with default parameters.
pub fn hash_password(password: &str) -> Result<String, PasswordHashError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

/// Verify a password against Argon2id v19 PHC string.
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, PasswordHashError> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
