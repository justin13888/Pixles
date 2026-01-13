#[cfg(feature = "auth")]
pub const ACCESS_TOKEN_EXPIRY: u64 = 60 * 15; // 15 minutes
#[cfg(feature = "auth")]
pub const REFRESH_TOKEN_EXPIRY: u64 = 60 * 60 * 24 * 7; // 7 days
#[cfg(feature = "auth")]
pub const TOTP_ISSUER: &str = "Pixles";
#[cfg(feature = "auth")]
pub const MAX_PASSKEYS_PER_USER: usize = 10;

#[cfg(feature = "upload")]
pub const MAX_FILE_SIZE: usize = 32 * 1024 * 1024 * 1024; // 32 GiB
#[cfg(feature = "upload")]
pub const MAX_CACHE_SIZE: usize = 64 * 1024 * 1024 * 1024; // 64 GiB
