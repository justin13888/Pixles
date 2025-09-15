use std::path::Path;
use std::{fs, io, ops::Deref};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::hash::get_file_hash;

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
    /// Generate file metadata from a path
    pub async fn from_file_path(path: &Path) -> io::Result<FileMetadata> {
        let metadata = fs::metadata(path)?;

        // Get file hash
        let hash_xxh3 = get_file_hash(path)?;

        // Get file size
        let size = metadata.len();

        // Convert path to string with POSIX separators
        let original_path = path.to_string_lossy().replace('\\', "/");

        // Extract filename from path
        let original_filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Get timestamps
        let created_timestamp = metadata
            .created()
            .map(DateTime::<Utc>::from)
            .unwrap_or_else(|_| Utc::now());

        let modified_timestamp = metadata
            .modified()
            .map(DateTime::<Utc>::from)
            .unwrap_or_else(|_| Utc::now());

        let import_timestamp = Utc::now();

        // Basic MIME type detection based on file extension
        let mime = detect_mime_type(&original_filename);

        Ok(FileMetadata {
            original_path,
            hash_xxh3: hash_xxh3.into(),
            size,
            mime,
            original_filename,
            created_timestamp,
            modified_timestamp,
            import_timestamp,
        })
    }

    /// Returns the original file name based on the original path.
    pub fn original_filename(&self) -> String {
        self.original_path
            .split('/')
            .next_back()
            .unwrap_or("unknown")
            .to_string()
    }
}

// TODO: vv Verify and extend
// TODO: Should probably move this to another library
/// Simple MIME type detection based on file extension
fn detect_mime_type(filename: &str) -> Option<String> {
    let extension = filename.rsplit('.').next()?.to_lowercase();

    match extension.as_str().to_lowercase().as_str() {
        // Images
        "jpg" | "jpeg" => Some("image/jpeg".to_string()),
        "png" => Some("image/png".to_string()),
        "gif" => Some("image/gif".to_string()),
        "webp" => Some("image/webp".to_string()),
        "svg" => Some("image/svg+xml".to_string()),
        "bmp" => Some("image/bmp".to_string()),
        "tiff" | "tif" => Some("image/tiff".to_string()),
        "ico" => Some("image/x-icon".to_string()),

        // Videos
        "mp4" => Some("video/mp4".to_string()),
        "avi" => Some("video/x-msvideo".to_string()),
        "mov" => Some("video/quicktime".to_string()),
        "wmv" => Some("video/x-ms-wmv".to_string()),
        "flv" => Some("video/x-flv".to_string()),
        "webm" => Some("video/webm".to_string()),
        "mkv" => Some("video/x-matroska".to_string()),

        // Audio
        "mp3" => Some("audio/mpeg".to_string()),
        "wav" => Some("audio/wav".to_string()),
        "flac" => Some("audio/flac".to_string()),
        "ogg" => Some("audio/ogg".to_string()),
        "aac" => Some("audio/aac".to_string()),
        "wma" => Some("audio/x-ms-wma".to_string()),

        // Documents
        "pdf" => Some("application/pdf".to_string()),
        "doc" => Some("application/msword".to_string()),
        "docx" => Some(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        ),
        "xls" => Some("application/vnd.ms-excel".to_string()),
        "xlsx" => {
            Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string())
        }
        "ppt" => Some("application/vnd.ms-powerpoint".to_string()),
        "pptx" => Some(
            "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string(),
        ),

        // Text
        "txt" => Some("text/plain".to_string()),
        "html" | "htm" => Some("text/html".to_string()),
        "css" => Some("text/css".to_string()),
        "js" => Some("application/javascript".to_string()),
        "json" => Some("application/json".to_string()),
        "xml" => Some("application/xml".to_string()),
        "csv" => Some("text/csv".to_string()),

        // Archives
        "zip" => Some("application/zip".to_string()),
        "rar" => Some("application/vnd.rar".to_string()),
        "7z" => Some("application/x-7z-compressed".to_string()),
        "tar" => Some("application/x-tar".to_string()),
        "gz" => Some("application/gzip".to_string()),

        _ => None,
    }
}
