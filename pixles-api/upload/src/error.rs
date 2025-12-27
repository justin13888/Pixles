use salvo::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UploadError {
    #[error("File exceeds size limit")]
    FileTooLarge,
    #[error("Cache is full")]
    CacheFull,
    #[error("File system error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Database error: {0}")]
    DbError(#[from] sea_orm::DbErr),
    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Valkey error: {0}")]
    ValkeyError(#[from] bb8_redis::redis::RedisError),
    #[error("RunError: {0}")]
    RunError(#[from] bb8_redis::bb8::RunError<bb8_redis::redis::RedisError>),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Session not found")]
    SessionNotFound,
    #[error("Upload already complete")]
    UploadComplete,
    #[error("Invalid offset: expected {expected}, got {actual}")]
    InvalidOffset { expected: u64, actual: u64 },
    #[error("Invalid upload: {0}")]
    InvalidUpload(String),
    #[error("Processing error: {0}")]
    ProcessingError(String),
    #[error("Parse error: {0}")]
    ParseError(#[from] std::string::ParseError),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[async_trait]
impl Writer for UploadError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let (status, message) = match self {
            UploadError::FileTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                String::from("File exceeds size limit"),
            ),
            UploadError::CacheFull => (
                StatusCode::INSUFFICIENT_STORAGE,
                String::from("Cache is full"),
            ),
            UploadError::IoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("File system error"),
            ),
            UploadError::DbError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Database error"),
            ),
            UploadError::SerdeError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Serialization error"),
            ),
            UploadError::ValkeyError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Cache error"),
            ),
            UploadError::RunError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Cache connection error"),
            ),
            UploadError::ConfigError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            UploadError::SessionNotFound => (
                StatusCode::NOT_FOUND,
                String::from("Upload session not found"),
            ),
            UploadError::UploadComplete => (
                StatusCode::CONFLICT,
                String::from("Upload already complete"),
            ),
            UploadError::InvalidOffset { expected, actual } => (
                StatusCode::CONFLICT,
                format!("Invalid offset. Expected {}, got {}", expected, actual),
            ),
            UploadError::InvalidUpload(msg) => (StatusCode::BAD_REQUEST, msg),
            UploadError::ProcessingError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Processing error"),
            ),
            UploadError::ParseError(_) => (StatusCode::BAD_REQUEST, String::from("Parse error")),
            UploadError::Unknown(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        res.status_code(status);
        res.render(Text::Plain(message));
    }
}
