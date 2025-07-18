use std::ops::Deref;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct HashData(u64);

impl std::fmt::Debug for HashData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl From<u64> for HashData {
    fn from(value: u64) -> Self {
        HashData(value)
    }
}

impl Deref for HashData {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Original file path formatted with POSIX path separators ("/")
    pub original_path: String,
    /// XXH3 Hash
    pub hash_xxh3: HashData,
    /// File size in bytes
    pub size: u64,
    /// Mime format if available
    pub mime: Option<String>,
    /// Original file name
    pub original_filename: String,
    /// Creation timestamp
    pub created_timestamp: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_timestamp: DateTime<Utc>,

    /// Import timestamp
    pub import_timestamp: DateTime<Utc>,
    // pub size: u64,
    // pub modified: u64,
}

impl FileMetadata {
    /// Returns the original file name based on the original path.
    pub fn original_filename(&self) -> String {
        self.original_path
            .split('/')
            .last()
            .unwrap_or("unknown")
            .to_string()
    }
}
