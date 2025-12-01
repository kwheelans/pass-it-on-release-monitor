use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Iden, Set};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::sea_query::OnConflict;
use tracing::debug;
use tracing::log::warn;
use crate::database::{monitors, MonitorEntity, MonitorActiveModel, MonitorModel};
use crate::monitors::Monitor;

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

pub async fn insert_monitor(
    db: &DatabaseConnection,
    monitor: Box<dyn Monitor>,
) -> Result<(), DbErr> {
    let conflict_id = OnConflict::column(monitors::Column::Id)
        .do_nothing()
        .to_owned();
    let conflict_name = OnConflict::column(monitors::Column::Name)
        .update_column(monitors::Column::Configuration)
        .to_owned();
    let monitor = MonitorActiveModel {
        id: Default::default(),
        name: Set(monitor.name()),
        monitor_type: Set(monitor.monitor_type()),
        configuration: Set(monitor.to_json()),
        version: Set("".to_string()),
        timestamp: Set(DateTimeUtc::default().into()),
    };
    let result = MonitorEntity::insert(monitor)
        .on_conflict(conflict_id)
        .on_conflict(conflict_name)
        .exec(db)
        .await;
    debug!("Insert Result: {:?}", result);
    insert_result(result)
}

fn insert_result<T>(result: Result<T, DbErr>) -> Result<(), DbErr> {
    match result {
        Ok(_) => Ok(()),
        Err(DbErr::RecordNotInserted) => {
            warn!("Record not inserted.");
            Ok(())
        }
        Err(e) => Err(e),
    }
}
