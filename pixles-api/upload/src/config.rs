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
    pub max_file_size: u64,
    /// Maximum cache size in bytes
    pub max_cache_size: u64,
    /// Sled database path
    pub db_path: PathBuf,
}

impl From<ServerConfig> for UploadServerConfig {
    fn from(config: ServerConfig) -> Self {
        Self {
            host: config.host,
            port: config.port,
            domain: config.domain,
            upload_dir: config.upload_dir,
            max_file_size: config.max_file_size,
            max_cache_size: config.max_cache_size,
            db_path: config.db_path,
        }
    }
}
