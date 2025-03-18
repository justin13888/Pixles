use std::{
    cell::LazyCell,
    fs,
    sync::{Arc, LazyLock, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::{config::UploadServerConfig, error::UploadError};

// File metadata model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMetadata {
    pub id: String,
    pub original_filename: String,
    pub content_type: String,
    pub size: usize,
    pub uploaded_at: u64,
    pub path: String,
}

// TODO: Might remove this
// Cache status tracker
static CACHE_SIZE: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));

/// Database for metadata storage
#[derive(Clone)]
pub struct FileDatabase {
    db: sled::Db,
    config: UploadServerConfig,
}

impl FileDatabase {
    pub async fn new(config: UploadServerConfig) -> Result<Self, sled::Error> {
        // Ensure the upload directory exists
        fs::create_dir_all(&config.upload_dir).map_err(|e| {
            sled::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create upload directory: {}", e),
            ))
        })?;

        // Ensure the database directory exists
        fs::create_dir_all(&config.sled_db_dir).map_err(|e| {
            sled::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create database directory: {}", e),
            ))
        })?;

        // Open the database
        let db = sled::open(&config.sled_db_dir)?;

        // Initialize cache size tracker
        if let Ok(Some(size_bytes)) = db.get("cache_size") {
            if let Ok(size_str) = std::str::from_utf8(&size_bytes) {
                if let Ok(size) = size_str.parse::<usize>() {
                    let mut cache_size = CACHE_SIZE.lock().unwrap();
                    *cache_size = size;
                }
            }
        }

        Ok(Self { db, config })
    }

    pub async fn save_metadata(&self, metadata: &FileMetadata) -> Result<(), UploadError> {
        let metadata_json = serde_json::to_string(metadata)
            .map_err(|_| UploadError::ParseError(String::from("Failed to serialize metadata")))?;

        self.db
            .insert(metadata.id.as_bytes(), metadata_json.as_bytes())?;

        // Update cache size
        let mut cache_size = CACHE_SIZE.lock().unwrap();
        *cache_size += metadata.size;
        self.db
            .insert("cache_size", cache_size.to_string().as_bytes())?;

        self.db.flush()?;
        Ok(())
    }

    pub fn get_metadata(&self, id: &str) -> Result<Option<FileMetadata>, UploadError> {
        match self.db.get(id.as_bytes())? {
            Some(data) => {
                let metadata_str = std::str::from_utf8(&data).map_err(|_| {
                    UploadError::ParseError(String::from("Invalid UTF-8 in metadata"))
                })?;

                let metadata: FileMetadata = serde_json::from_str(metadata_str).map_err(|_| {
                    UploadError::ParseError(String::from("Failed to deserialize metadata"))
                })?;

                Ok(Some(metadata))
            }
            None => Ok(None),
        }
    }

    pub fn get_all_metadata(&self) -> Result<Vec<FileMetadata>, UploadError> {
        let mut result = Vec::new();

        for item in self.db.iter() {
            let (key, value) = item?;

            // Skip the cache_size entry
            if key.as_ref() == b"cache_size" {
                continue;
            }

            let metadata_str = std::str::from_utf8(&value)
                .map_err(|_| UploadError::ParseError(String::from("Invalid UTF-8 in metadata")))?;

            let metadata: FileMetadata = serde_json::from_str(metadata_str).map_err(|_| {
                UploadError::ParseError(String::from("Failed to deserialize metadata"))
            })?;

            result.push(metadata);
        }

        Ok(result)
    }

    pub async fn remove_file(&self, id: &str) -> Result<bool, UploadError> {
        if let Some(metadata) = self.get_metadata(id)? {
            // Remove file from disk
            fs::remove_file(&metadata.path)?;

            // Update cache size
            let mut cache_size = CACHE_SIZE.lock().unwrap();
            *cache_size = cache_size.saturating_sub(metadata.size);
            self.db
                .insert("cache_size", cache_size.to_string().as_bytes())?;

            // Remove from database
            self.db.remove(id.as_bytes())?;
            self.db.flush()?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn would_exceed_cache_limit(&self, file_size: usize) -> bool {
        let cache_size = CACHE_SIZE.lock().unwrap();
        *cache_size + file_size > self.config.max_cache_size
    }
}
