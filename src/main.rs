mod cli;
mod configuration;
mod error;
mod monitors;

use crate::cli::CliArgs;
use crate::configuration::MonitorConfigFileParser;
use crate::error::Error;
use crate::monitors::monitor;
use clap::Parser;
use pass_it_on::start_client;
use std::process::ExitCode;
use std::str::FromStr;
use tokio::sync::mpsc;
use tracing::level_filters::LevelFilter;
use tracing::log::debug;
use tracing::{error, info};

#[tokio::main]
async fn main() -> ExitCode {
    let args = CliArgs::parse();
    let verbosity = args.verbosity.unwrap_or(
        LevelFilter::from_str(std::env::var("VERBOSITY").unwrap_or_default().as_str())
            .unwrap_or(LevelFilter::INFO),
    );

    // Configure logging
    tracing_subscriber::fmt().with_max_level(verbosity).init();
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
    let config_path = args.config;
    if !config_path.is_file() {
        return Err(Error::MissingConfiguration(format!(
            "Configuration file {} is not a file or does not exist",
            config_path.to_string_lossy()
        )));
    }

    let config = MonitorConfigFileParser::try_from(std::fs::read_to_string(config_path)?.as_str())?;
    let (interface_tx, interface_rx) = mpsc::channel(100);
    let persist_path = match config.global.persist {
        true => Some(config.global.data_path.into()),
        false => None,
    };
    debug!("Persistence settings: {:?}", persist_path);

    tokio::spawn(async move {
        monitor(config.monitors.monitor, interface_tx.clone(), persist_path).await
    });

    start_client(config.client.try_into()?, interface_rx, None, None).await?;
    Ok(())
}
