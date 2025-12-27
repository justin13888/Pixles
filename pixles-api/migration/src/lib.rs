pub use sea_orm_migration::prelude::*;

mod m20250210_023042_initialize;
mod m20251211_000000_auth_completion;
mod m20251226_010000_upload_schema;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250210_023042_initialize::Migration),
            Box::new(m20251211_000000_auth_completion::Migration),
            Box::new(m20251226_010000_upload_schema::Migration),
        ]
    } // TODO: Make it easier to read. Merge these migrations because we don't need compatibility until stabilization.
}
