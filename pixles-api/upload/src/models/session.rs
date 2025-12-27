use chrono::{DateTime, Utc};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UploadSession {
    pub id: String,
    pub user_id: String,

    // Metadata from creation request
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub total_size: Option<u64>,

    // Upload state
    pub received_bytes: u64,
    pub status: UploadStatus,

    // Expected hash (xxh3) for final validation
    pub expected_hash: u64,

    // Optional Album ID to link the upload to
    pub album_id: Option<String>,

    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum UploadStatus {
    /// Active with no active upload
    Pending,
    /// Active with an active upload
    Uploading,
    /// Waiting for processing to complete
    WaitingForProcessing,
    /// Waiting for confirmation
    WaitingForConfirmation,
    /// Completed successfully
    Completed,
    /// Failed to process
    FailedProcessing,
}
