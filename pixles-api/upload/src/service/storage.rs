use crate::{config::UploadServerConfig, error::UploadError};
use std::path::PathBuf;

#[derive(Clone)]
pub struct StorageService {
    config: UploadServerConfig,
}

impl StorageService {
    pub fn new(config: UploadServerConfig) -> Self {
        Self { config }
    }

    pub fn get_upload_dir(&self, upload_id: &str) -> PathBuf {
        self.config.upload_dir.join(upload_id)
    }

    pub fn get_chunks_dir(&self, upload_id: &str) -> PathBuf {
        self.get_upload_dir(upload_id).join("chunks")
    }

    pub fn get_state_path(&self, upload_id: &str) -> PathBuf {
        self.get_upload_dir(upload_id).join("state.json")
    }

    pub fn get_chunk_path(&self, upload_id: &str, chunk_idx: usize) -> PathBuf {
        self.get_chunks_dir(upload_id)
            .join(format!("{:06}", chunk_idx))
    }

    /// Initializes the upload directory structure
    pub async fn init_upload_dir(&self, upload_id: &str) -> Result<(), UploadError> {
        let chunks_dir = self.get_chunks_dir(upload_id);
        tokio::fs::create_dir_all(&chunks_dir).await?;
        Ok(())
    }

    /// Combines chunks into a final file
    /// Uses reflink for efficient copy-on-write filesystems, falls back to regular copy.
    pub async fn combine_chunks(
        &self,
        upload_id: &str,
        final_filename: &str,
        num_chunks: usize,
    ) -> Result<PathBuf, UploadError> {
        let target_path = self.get_upload_dir(upload_id).join(final_filename);

        // Collect chunk paths
        let mut chunk_paths = Vec::with_capacity(num_chunks);
        for i in 0..num_chunks {
            chunk_paths.push(self.get_chunk_path(upload_id, i));
        }

        // Combine chunks using reflink when possible, fallback to copy
        // This is done in a blocking context since reflink::reflink_or_copy is sync
        let target = target_path.clone();
        tokio::task::spawn_blocking(move || {
            use std::fs::OpenOptions;
            use std::io::Write;

            let mut target_file = OpenOptions::new()
                .create_new(true) // Ensure file doesn't overwrite
                .write(true)
                .open(&target)?;

            for chunk_path in chunk_paths {
                // TODO: Attempt reflink for CoW filesystems (btrfs, xfs, etc.)
                // We can attempt reflink because chunks are assumed to be 4KB aligned.
                // TODO: On upload API startup, it should check if reflink is supported.
                let chunk_data = std::fs::read(&chunk_path)?;
                target_file.write_all(&chunk_data)?;
            }

            target_file.sync_all()?;
            Ok::<PathBuf, std::io::Error>(target)
        })
        .await
        .map_err(|e| UploadError::Unknown(format!("Task join error: {}", e)))?
        .map_err(UploadError::IoError)
    }
}
