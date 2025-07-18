use std::path::Path;
use std::{fs, io};

use chrono::{DateTime, Utc};

use crate::models::file::FileMetadata;
use crate::utils::metadata::get_file_hash;

/// Get metadata of a file
pub fn get_file_metadata(path: &Path) -> io::Result<FileMetadata> {
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

// TODO: vv Verify and extend
/// Simple MIME type detection based on file extension
fn detect_mime_type(filename: &str) -> Option<String> {
    let extension = filename.rsplit('.').next()?.to_lowercase();

    match extension.as_str().to_lowercase().as_str()
    {
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
        "xlsx" =>
        {
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
