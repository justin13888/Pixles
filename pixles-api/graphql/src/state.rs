use sea_orm::DatabaseConnection;

use crate::schema::AppSchema;

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub conn: DatabaseConnection,
}
