use std::path::PathBuf;

use environment::ServerConfig;

#[derive(Clone)]
pub struct UploadServerConfig {
    pub host: String,
    pub port: u16,
    pub domain: String,

    /// Upload directory
    pub upload_dir: PathBuf,
    /// Maximum file size in bytes
    pub max_file_size: usize,
    /// Maximum cache size in bytes
    pub max_cache_size: usize,
    /// Sled database directory
    pub sled_db_dir: PathBuf,
}

impl From<&ServerConfig> for UploadServerConfig {
    fn from(config: &ServerConfig) -> Self {
        Self {
            host: config.host.clone(),
            port: config.port,
            domain: config.domain.clone(),
            upload_dir: config.upload_dir.clone(),
            max_file_size: config.max_file_size,
            max_cache_size: config.max_cache_size,
            sled_db_dir: config.sled_db_dir.clone(),
        }
    }
}
