use std::{
    fs,
    io::Write,
    path::Path,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};
use tracing::{debug, info};
use uuid::Uuid;

use crate::{error::UploadError, metadata::FileMetadata, state::AppState};

// TODO: Verify
pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, String), UploadError> {
    // TODO: remove
    info!("Uploading file");
    let mut uploaded_files = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| UploadError::ParseError(format!("Failed to process multipart form: {}", e)))?
    {
        // Extract field metadata
        let file_name = match field.file_name() {
            Some(name) => name.to_string(),
            None => continue, // Skip fields without a filename
        };

        let content_type = field
            .content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // Generate a unique ID and determine storage path
        let file_id = Uuid::new_v4().to_string();
        let storage_path = Path::new(&state.config.upload_dir).join(&file_id);

        // TODO: remove
        debug!("Processing file: {:?}, {:?}", file_id, storage_path);

        // Read the file data and check size constraints
        let data = field
            .bytes()
            .await
            .map_err(|_| UploadError::ParseError(String::from("Failed to read file data")))?;

        // Check file size
        if data.len() > state.config.max_file_size {
            return Err(UploadError::FileTooLarge);
        }

        // Check if adding this file would exceed the cache limit
        if state.file_db.would_exceed_cache_limit(data.len()) {
            return Err(UploadError::CacheFull);
        }

        // Save the file
        let mut file = fs::File::create(&storage_path)?;
        file.write_all(&data)?;

        // Create and save metadata
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let metadata = FileMetadata {
            id: file_id.clone(),
            original_filename: file_name,
            content_type,
            size: data.len(),
            uploaded_at: now,
            path: storage_path.to_string_lossy().to_string(),
        };

        state.file_db.save_metadata(&metadata).await?;

        uploaded_files.push(file_id);
    }

    if uploaded_files.is_empty() {
        Ok((
            StatusCode::BAD_REQUEST,
            String::from("No files were uploaded"),
        ))
    } else if uploaded_files.len() == 1 {
        Ok((
            StatusCode::CREATED,
            format!("File uploaded with ID: {}", uploaded_files[0]),
        ))
    } else {
        Ok((
            StatusCode::CREATED,
            format!("Files uploaded with IDs: {}", uploaded_files.join(", ")),
        ))
    }
}
