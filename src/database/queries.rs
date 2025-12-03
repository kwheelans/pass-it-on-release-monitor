use crate::database::{MonitorActiveModel, MonitorEntity, MonitorModel, monitors};
use crate::monitors::Monitor;
use sea_orm::prelude::{DateTimeUtc, Expr};
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait, ModelTrait, Set,
};
use std::fmt::Debug;
use tracing::debug;
use tracing::log::warn;

pub async fn select_all_monitors(db: &DatabaseConnection) -> Result<Vec<MonitorModel>, DbErr> {
    let records: Vec<MonitorModel> = MonitorEntity::find().all(db).await?;
    Ok(records)
}

pub async fn select_one_monitor(
    db: &DatabaseConnection,
    id: i64,
) -> Result<Option<MonitorModel>, DbErr> {
    let record = MonitorEntity::find_by_id(id).one(db).await?;
    Ok(record)
}

pub async fn add_monitor(db: &DatabaseConnection, monitor: Box<dyn Monitor>) -> Result<(), DbErr> {
    let conflict = OnConflict::column(monitors::Column::Name)
        .do_nothing()
        .to_owned();
    insert_monitor(db, monitor, conflict).await
}

pub async fn add_static_monitor(
    db: &DatabaseConnection,
    monitor: Box<dyn Monitor>,
) -> Result<(), DbErr> {
    let conflict = OnConflict::column(monitors::Column::Name)
        .action_and_where(Expr::col(monitors::Column::MonitorType).is(monitor.monitor_type()))
        .update_column(monitors::Column::Configuration)
        .to_owned();
    insert_monitor(db, monitor, conflict).await
}

async fn insert_monitor(
    db: &DatabaseConnection,
    monitor: Box<dyn Monitor>,
    on_conflict: OnConflict,
) -> Result<(), DbErr> {
    let monitor = MonitorActiveModel {
        id: Default::default(),
        name: Set(monitor.name()),
        monitor_type: Set(monitor.monitor_type()),
        configuration: Set(monitor.inner_to_json()),
        version: Set("".to_string()),
        timestamp: Set(DateTimeUtc::default().into()),
    };
    let result = MonitorEntity::insert(monitor)
        .on_conflict(on_conflict)
        .exec(db)
        .await;
    debug!("Insert Result: {:?}", result);
    insert_result(result)
}

fn insert_result<T: Debug>(result: Result<T, DbErr>) -> Result<(), DbErr> {
    match result {
        Ok(r) => {
            debug!("Insert Result: {:?}", r);
            Ok(())
        }
        Err(DbErr::RecordNotInserted) => {
            warn!("Record not inserted.");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub async fn update_monitor(
    db: &DatabaseConnection,
    model: MonitorActiveModel,
) -> Result<(), DbErr> {
    model.update(db).await?;
    Ok(())
}

pub async fn delete_monitor(db: &DatabaseConnection, id: i64) -> Result<(), DbErr> {
    let monitor = MonitorEntity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find record.".to_owned()))?;
    let result = monitor.delete(db).await;
    debug!("Delete Result: {:?}", result);
    Ok(())
}
