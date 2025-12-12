use environment::ServerConfig;

#[derive(Clone)]
pub struct GraphqlServerConfig {}
// TODO: Flesh this out ^^

impl From<&ServerConfig> for GraphqlServerConfig {
    fn from(config: &ServerConfig) -> Self {
        Self {}
    }
}
