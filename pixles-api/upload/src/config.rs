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

/// Validate the configuration. Returns error if configuration is valid.
/// Returns a list of warnings if configuration is valid but has potential issues.
pub fn validate_config(config: &UploadServerConfig) -> Result<Vec<String>, String> {
    let mut warnings = vec![];
    if config.max_file_size >= config.max_cache_size {
        return Err(String::from(
            "max_file_size must be less than max_cache_size",
        ));
    }

    // Warn max_file_size allows < 10 concurrent files
    if config.max_cache_size / config.max_file_size < 10 {
        warnings.push(
            "Based on current max_cache_size, max_file_size allows < 10 concurrent files"
                .to_string(),
        );
    }

    // Warn if upload_dir is a non-empty directory
    if config.upload_dir.is_dir()
        && config
            .upload_dir
            .read_dir()
            .map_err(|e| format!("Unable to read upload directory: {e:?}"))?
            .count()
            > 0
    {
        warnings.push("upload_dir is non-empty. This may be from server restarts.".to_string());
    }

    // Warn if sled_db_dir is an existing directory
    if config.sled_db_dir.is_dir() {
        warnings.push(
            "sled_db_dir is an existing directory. This may be from server restarts.".to_string(),
        );
    }

    Ok(warnings)
}
