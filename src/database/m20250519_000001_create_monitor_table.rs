use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Monitors::Table)
                    .if_not_exists()
                    .col(pk_uuid(Monitors::Id))
                    .col(string(Monitors::MonitorType))
                    .col(string_null(Monitors::Version))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Monitors::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Monitors {
    Table,
    Id,
    MonitorType,
    Version,
}
