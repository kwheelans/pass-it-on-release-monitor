use sea_orm_migration::prelude::*;

pub mod version;
pub mod m20250519_000001_create_version_table;

pub use version::Entity as VersionEntity;
pub use version::ActiveModel as VersionEntityActiveModel;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250519_000001_create_version_table::Migration)]
    }
}
