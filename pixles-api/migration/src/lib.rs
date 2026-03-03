pub use sea_orm_migration::prelude::*;

mod m20250210_000000_initial_schema;
mod m20250302_000000_add_registered_via;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250210_000000_initial_schema::Migration),
            Box::new(m20250302_000000_add_registered_via::Migration),
        ]
    }
}
