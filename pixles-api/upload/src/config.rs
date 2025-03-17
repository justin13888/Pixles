use environment::ServerConfig;

#[derive(Clone)]
pub struct UploadServerConfig {
    pub host: String,
    pub port: u16,
    pub domain: String,
}

impl From<ServerConfig> for UploadServerConfig {
    fn from(config: ServerConfig) -> Self {
        Self {
            host: config.host,
            port: config.port,
            domain: config.domain,
        }
    }
}
