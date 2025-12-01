use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "monitors")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,
    pub monitor_type: String,
    pub configuration: String,
    pub version: String,
    pub timestamp: ChronoUnixTimestamp,
}

impl ActiveModelBehavior for ActiveModel {}
