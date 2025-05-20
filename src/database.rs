use sea_orm_migration::prelude::*;

pub mod monitors;
pub mod m20250519_000001_create_monitor_table;

pub use monitors::Entity as MonitorsEntity;
pub use monitors::ActiveModel as MonitorsEntityActiveModel;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250519_000001_create_monitor_table::Migration)]
    }
}
