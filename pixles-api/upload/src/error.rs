use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
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
    DatabaseError(#[from] sled::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
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
            UploadError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Database error"),
            ),
            UploadError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, message).into_response()
    }
}
