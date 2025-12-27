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
    /// Attempts reflink first, falls back to copy.
    pub async fn combine_chunks(
        &self,
        upload_id: &str,
        final_filename: &str,
        num_chunks: usize,
    ) -> Result<PathBuf, UploadError> {
        // Warning: This implementation of combine_chunks relies on blocking reflink calls.
        // In a high-throughput async context, we should spawn_blocking or use async fs.

        let target_path = self.get_upload_dir(upload_id).join(final_filename);

        let mut target_file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&target_path)
            .await?;

        for i in 0..num_chunks {
            let chunk_path = self.get_chunk_path(upload_id, i);
            let mut chunk_file = tokio::fs::File::open(&chunk_path).await?;
            tokio::io::copy(&mut chunk_file, &mut target_file).await?;
        }

        Ok(target_path)
    }
}
