use environment::ServerConfig;

#[derive(Clone)]
pub struct MetadataServerConfig {
}
// TODO: flesh this out ^^

impl From<&ServerConfig> for MetadataServerConfig {
    fn from(config: &ServerConfig) -> Self {
        Self {
            
        }
    }
}
