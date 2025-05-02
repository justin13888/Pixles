mod mutation;
mod query;

pub use mutation::*;
pub use query::*;

/// Returns true if username follows username rules
pub fn is_valid_username(username: &str) -> bool {
    // Users cannot be "admin", "root", "pixles", "admin*", etc.
    if username.eq_ignore_ascii_case("admin")
        || username.eq_ignore_ascii_case("root")
        || username.eq_ignore_ascii_case("pixles")
    {
        return false;
    }

    if username.to_ascii_lowercase().starts_with("admin") {
        return false;
    }

    true
}

/// Returns true if email is valid
pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') // TODO: Add more validation rules
}

/// Returns true if password is valid
pub fn is_valid_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }

    Ok(())
}

// // Returns normalized username
// pub fn normalize_username(username: &str) -> String {
//     username.to_ascii_lowercase() // TODO: Check if it is needed since db could index normalized version
// }

// Returns normalized email
pub fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
} // TODO: Check this is what we want
