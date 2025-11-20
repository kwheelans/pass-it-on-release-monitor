use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "version")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub monitor_id: String,
    #[sea_orm(nullable)]
    pub version: String,
}

impl ActiveModelBehavior for ActiveModel {}
