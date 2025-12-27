use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadSession {
    pub id: String,
    pub user_id: String,

    // Metadata from creation request
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub total_size: Option<u64>,

    // Upload state
    pub received_bytes: u64,
    pub is_complete: bool,

    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}
