use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{config::GraphqlServerConfig, schema::AppSchema};

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub conn: Arc<DatabaseConnection>,
    pub config: GraphqlServerConfig,
}
