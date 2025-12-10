mod cli;
mod configuration;
mod database;
mod error;
mod monitors;
mod ui;

use crate::cli::CliArgs;
use crate::configuration::{ReleaseMonitorConfiguration, get_css_path};
use crate::database::MonitorEntity;
use crate::database::queries::add_static_monitor;
use crate::error::Error;
use crate::monitors::start_monitoring;
use crate::ui::handlers::{AppState, serve_web_ui};
use clap::Parser;
use pass_it_on::start_client;
use sea_orm::Database;
use std::process::ExitCode;
use std::str::FromStr;
use tokio::sync::mpsc;
use tracing::level_filters::LevelFilter;
use tracing::log::debug;
use tracing::{error, info};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const SQLITE_MEMORY: &str = "sqlite::memory:";

#[tokio::main]
async fn main() -> ExitCode {
    let args = CliArgs::parse();
    let verbosity = args.verbosity.unwrap_or(
        LevelFilter::from_str(std::env::var("VERBOSITY").unwrap_or_default().as_str())
            .unwrap_or(LevelFilter::INFO),
    );
    let sqlx_log_level = match verbosity {
        LevelFilter::TRACE => LevelFilter::TRACE,
        LevelFilter::DEBUG => LevelFilter::DEBUG,
        _ => LevelFilter::WARN,
    };

    // Configure logging
    let log_filter = Targets::default()
        .with_target("pass_it_on_release_monitor", verbosity)
        .with_target("sqlx", sqlx_log_level)
        .with_default(LevelFilter::INFO);
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(log_filter)
        .init();
    info!("Verbosity set to {}", verbosity);

    match run(args).await {
        Err(error) => {
            error!("{}", error);
            ExitCode::FAILURE
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}

async fn run(args: CliArgs) -> Result<(), Error> {
    // Get configuration
    let config_path = args.config;
    if !config_path.is_file() {
        return Err(Error::MissingConfiguration(format!(
            "Configuration file {} is not a file or does not exist",
            config_path.to_string_lossy()
        )));
    }
    let config =
        ReleaseMonitorConfiguration::try_from(std::fs::read_to_string(config_path)?.as_str())?;
    debug!("{:?}", config);

    // Get database connection
    let db_uri = match config.global.persist {
        true => config.global.db_uri(),
        false => SQLITE_MEMORY.into(),
    };
    let db = Database::connect(db_uri).await?;
    db.get_schema_builder()
        .register(MonitorEntity)
        .sync(&db)
        .await?;

    // Set CSS Path
    let css_path = get_css_path(config.webui.pico_css_base_path, config.webui.pico_css_color);

    // Initialize state & listener for Axum
    let state = AppState::new(db, css_path);
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.webui.listen_address, config.webui.port
    ))
    .await?;
    let db = state.db().clone();
    info!("Listening on: {}", listener.local_addr()?);

    // Insert initial monitors from configuration if they do not exist
    for monitor in &config.monitors.monitor {
        add_static_monitor(&db, monitor.clone()).await?
    }

    // Setup message channel
    let (interface_tx, interface_rx) = mpsc::channel(100);

    // Start monitor task
    tokio::spawn(async move { start_monitoring(&db, config.global, interface_tx.clone()).await });

    // Start Web UI
    tokio::spawn(async move { serve_web_ui(state, listener).await });

    // Start Pass-It-On client
    start_client(config.client.try_into()?, interface_rx, None, None).await?;
    Ok(())
}
