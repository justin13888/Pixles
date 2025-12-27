use crate::config::UploadServerConfig;
use crate::error::UploadError;
use crate::models::session::{UploadSession, UploadStatus};
use crate::service::processing::ProcessingService;
use crate::service::storage::StorageService;
use crate::session::UploadSessionManager;
use chrono::Utc;
use nanoid::nanoid;
use pixles_core::utils::hash::get_file_hash;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};
use std::clone::Clone;
use tokio::fs;

use entity::asset;
use service::album as AlbumService;

#[derive(Clone)]
pub struct UploadService {
    config: UploadServerConfig,
    storage: StorageService,
    session_manager: UploadSessionManager,
    processing_service: ProcessingService,
    conn: DatabaseConnection,
}

impl UploadService {
    pub fn new(
        config: UploadServerConfig,
        storage: StorageService,
        session_manager: UploadSessionManager,
        conn: DatabaseConnection,
    ) -> Self {
        let processing_service = ProcessingService::new();
        Self {
            config,
            storage,
            session_manager,
            processing_service,
            conn,
        }
    }

    pub async fn create_session(
        &self,
        user_id: &str,
        filename: Option<String>,
        content_type: Option<String>,
        total_size: Option<u64>,
        expected_hash: u64,
        album_id: Option<String>,
    ) -> Result<UploadSession, UploadError> {
        let upload_id = nanoid!();

        // Validate Album access if provided
        if let Some(album_id) = &album_id {
            match AlbumService::Query::get_album_access(&self.conn, user_id, album_id).await {
                Ok(access) => {
                    let has_write_access = access.map_or(false, |a| a.is_write());
                    if !has_write_access {
                        return Err(UploadError::InvalidUpload(
                            "Album access denied".to_string(),
                        ));
                    }
                }
                Err(e) => {
                    return Err(UploadError::InvalidUpload(e.to_string()));
                }
            }
        }

        let session = UploadSession {
            id: upload_id.clone(),
            user_id: user_id.to_string(),
            filename,
            content_type,
            total_size,
            received_bytes: 0,
            status: UploadStatus::Pending,
            expected_hash,
            album_id,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
        };

        // Init storage
        self.storage.init_upload_dir(&upload_id).await?;

        // Create session in Redis (atomic HSET)
        self.session_manager.create(&session).await?;

        // Save local backup state
        self.save_local_state(&session).await?;

        Ok(session)
    }

    pub async fn get_session(&self, upload_id: &str) -> Result<Option<UploadSession>, UploadError> {
        self.session_manager.get(upload_id).await
    }

    pub async fn append_chunk(
        &self,
        upload_id: &str,
        data: bytes::Bytes,
        offset: u64,
    ) -> Result<UploadSession, UploadError> {
        // Get current session state atomically
        let session = self
            .get_session(upload_id)
            .await?
            .ok_or(UploadError::SessionNotFound)?;

        if session.status == UploadStatus::Completed {
            return Err(UploadError::UploadComplete);
        }

        // Validate offset matches current received_bytes
        if offset != session.received_bytes {
            return Err(UploadError::InvalidOffset {
                expected: session.received_bytes,
                actual: offset,
            });
        }

        let chunk_len = data.len() as u64;

        // Validate size limit before writing
        let new_size = session.received_bytes + chunk_len;
        if let Some(total) = session.total_size
            && new_size > total
        {
            return Err(UploadError::InvalidUpload(
                "Upload exceeds declared total size".to_string(),
            ));
        }
        if new_size > self.config.max_file_size as u64 {
            return Err(UploadError::FileTooLarge);
        }

        // Count existing chunks to determine next chunk index
        let chunk_count = self.count_chunks(upload_id).await?;

        // Write chunk to disk
        let chunk_path = self.storage.get_chunk_path(upload_id, chunk_count);
        fs::write(&chunk_path, &data).await?;

        // Atomically increment received_bytes in Redis
        let new_received_bytes = self
            .session_manager
            .increment_received_bytes(upload_id, chunk_len)
            .await?;

        // Re-fetch session to get updated state
        let updated_session = UploadSession {
            received_bytes: new_received_bytes,
            ..session
        };

        // Save local backup state with updated bytes
        self.save_local_state(&updated_session).await?;

        Ok(updated_session)
    }

    /// Finalize an upload, committing uploaded data to storage and processing.
    pub async fn finalize_upload(&self, upload_id: &str) -> Result<asset::Model, UploadError> {
        let session = self
            .get_session(upload_id)
            .await?
            .ok_or(UploadError::SessionNotFound)?;

        if let Some(total) = session.total_size {
            if session.received_bytes != total {
                return Err(UploadError::InvalidUpload(format!(
                    "Upload not complete: received {} of {}",
                    session.received_bytes, total
                )));
            }
        } else {
            return Err(UploadError::InvalidUpload(
                "Cannot finalize upload without known total size".to_string(),
            ));
        }

        // Mark session as complete atomically
        self.session_manager
            .update_status(upload_id, UploadStatus::WaitingForProcessing)
            .await?;

        // Combine chunks
        let filename = session
            .filename
            .clone()
            .unwrap_or_else(|| format!("{}.bin", upload_id));
        let num_chunks = self.count_chunks(upload_id).await?;
        let final_path = self
            .storage
            .combine_chunks(upload_id, &filename, num_chunks)
            .await?;

        // Validate Hash - fail early and cleanup if mismatch
        let hash = get_file_hash(&final_path)?;

        if hash != session.expected_hash {
            tracing::debug!(
                "Hash mismatch for upload {upload_id} (expected: {:016x}, actual: {:016x})",
                session.expected_hash,
                hash
            );
            // Hash mismatch - cleanup files and fail
            let _ = self.cancel_upload(upload_id).await;
            return Err(UploadError::ChecksumMismatch {
                expected: format!("{:016x}", session.expected_hash),
                actual: format!("{:016x}", hash),
            });
        }

        // Extract Metadata
        let metadata = self
            .processing_service
            .extract_metadata(&final_path)
            .await
            .map_err(|e| UploadError::ProcessingError(e.to_string()))?;

        // DB Insert with Transaction
        let txn = self.conn.begin().await?;

        let user_id = &session.user_id;

        // Get/Create Owner
        let owner_service = crate::service::owner::OwnerService::new(self.conn.clone());
        let owner_id = owner_service
            .get_or_create_owner(&[user_id.clone()], &txn)
            .await?;

        // Determine mime
        let mime = session
            .content_type
            .clone()
            .unwrap_or_else(|| "application/octet-stream".to_string());
        let asset_type = if mime.starts_with("image/") {
            asset::AssetType::Photo
        } else if mime.starts_with("video/") {
            asset::AssetType::Video
        } else {
            asset::AssetType::Photo // default
        };

        let asset = asset::ActiveModel {
            id: Set(nanoid!()),
            owner_id: Set(owner_id),
            album_id: Set(session.album_id),
            asset_type: Set(asset_type),
            original_filename: Set(session.filename.unwrap_or_default()),
            file_size: Set(session.received_bytes as i64),
            file_hash: Set(hash as i64),
            content_type: Set(session.content_type.unwrap_or_default()),
            uploaded_at: Set(Utc::now()),
            modified_at: Set(Utc::now().into()),
            width: Set(metadata.width),
            height: Set(metadata.height),
            date: Set(metadata.date),
            ..Default::default()
        };

        let asset_res = asset.insert(&txn).await?;

        txn.commit().await?;

        Ok(asset_res)
    }

    /// Count number of chunks belonging to an upload
    async fn count_chunks(&self, upload_id: &str) -> tokio::io::Result<usize> {
        let chunks_dir = self.storage.get_chunks_dir(upload_id);
        let mut chunk_count = 0;
        let mut entries = fs::read_dir(&chunks_dir).await?;
        while (entries.next_entry().await?).is_some() {
            chunk_count += 1;
        }
        Ok(chunk_count)
    }

    /// Save local state of upload session
    async fn save_local_state(&self, session: &UploadSession) -> Result<(), UploadError> {
        let path = self.storage.get_state_path(&session.id);
        let content = serde_json::to_string(session)?;
        fs::write(path, content).await?;
        Ok(())
    }

    pub async fn cancel_upload(&self, upload_id: &str) -> Result<(), UploadError> {
        // 1. Remove from Valkey
        self.session_manager.delete(upload_id).await?;

        // 2. Delete local files
        let upload_dir = self.storage.get_upload_dir(upload_id);
        if upload_dir.exists() {
            fs::remove_dir_all(&upload_dir).await?;
        }

        Ok(())
    }
}
