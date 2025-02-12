use async_graphql::*;
use chrono::{DateTime, Utc};

use crate::schema::{user::User, Tag};

/// Asset Metadata
#[derive(SimpleObject)]
pub struct AssetMetadata {
    id: ID,
    user: User,
    width: i32,
    height: i32,
    date: DateTime<Utc>,
    uploaded_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
    /// List of tags
    tags: Vec<Tag>,
}

#[derive(SimpleObject)]
pub struct UploadSession {
    pub id: ID,
    /// User that created the upload session
    pub user: User,
    /// Method used to upload the asset
    pub method: UploadMethod,
    /// Destination of album. None if it is not associated
    pub album_id: Option<String>,
    /// Status of the upload session
    pub status: UploadStatus,
    /// Date when the upload session was created
    pub created_at: DateTime<Utc>,
    /// Date when the upload session automatically expires
    pub expires_at: DateTime<Utc>,
}

#[derive(InputObject)]
pub struct UploadSessionFilter {
    /// Album ID
    pub album_id: Option<ID>,
    /// Method
    pub method: Option<UploadMethod>,
    /// Statuses
    pub status: Option<Vec<UploadStatus>>,
    /// Expired
    pub expired: Option<bool>,
}

// TODO: Implement
#[derive(InputObject)]
pub struct CreateUploadSessionInput {
    /// Method used to upload the asset
    pub method: UploadMethod,
    /// Destination of album
    pub album_id: Option<ID>,
}

#[derive(InputObject)]
pub struct CreateAssetInput {
    /// ID of the upload session
    pub session_id: ID,
    /// ID of the album to add the asset to
    pub album_id: ID,
    // TODO: Add metadata necessary for grouping
    // TODO: Add any other relevant metadata
}

// TODO: Implement
#[derive(InputObject)]
pub struct UpdateAssetInput {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AssetType {
    Image,
    Video,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UploadMethod {
    /// Direct upload via GraphQL Upload scalar
    Direct,
    /// Upload via Multipart
    Multipart,
    /// Upload resumably via tus
    Tus,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
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

/// Status of the upload session completion
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UploadSessionCompletionStatus {
    /// Session does not exist
    DoesNotExist,
    /// Upload is still in progress
    InProgress,
    /// Closed successfully
    Success,
    /// Unknown error
    Unknown,
}

/// Filter for assets
#[derive(InputObject)]
pub struct AssetFilter {
    /// User ID
    pub user_id: Option<ID>,
    /// Tags
    pub tags: Option<Vec<String>>,
    /// Minimum date
    pub date_min: Option<DateTime<Utc>>,
    /// Maximum date
    pub date_max: Option<DateTime<Utc>>,
    /// Minimum uploaded at date
    pub uploaded_at_min: Option<DateTime<Utc>>,
    /// Maximum uploaded at date
    pub uploaded_at_max: Option<DateTime<Utc>>,
    /// Minimum modified at date
    pub modified_at_min: Option<DateTime<Utc>>,
    /// Maximum modified at date
    pub modified_at_max: Option<DateTime<Utc>>,
}
