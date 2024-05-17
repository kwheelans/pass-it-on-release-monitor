mod cli;
mod configuration;
mod error;
mod monitors;

use crate::cli::CliArgs;
use crate::configuration::MonitorConfigFileParser;
use crate::error::Error;
use crate::monitors::monitor;
use clap::Parser;
use log::{error, info, LevelFilter};
use pass_it_on::start_client;
use std::process::ExitCode;
use std::str::FromStr;
use tokio::sync::mpsc;
const LOG_TARGET: &str = "pass_it_on_release_monitor";

#[tokio::main]
async fn main() -> ExitCode {
    let args = CliArgs::parse();
    let verbosity = args.verbosity.unwrap_or(
        LevelFilter::from_str(std::env::var("VERBOSITY").unwrap_or_default().as_str())
            .unwrap_or(LevelFilter::Info),
    );

    // Configure logging
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .env()
        .with_module_level(LOG_TARGET, verbosity)
        .with_colors(true)
        .init()
        .unwrap();
    info!("Verbosity set to {}", verbosity);

    match run(args).await {
        Err(error) => {
            error!(target: LOG_TARGET, "{}", error);
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

    tokio::spawn(async move { monitor(config.monitors.monitor, interface_tx.clone()).await });

    start_client(config.client.try_into()?, interface_rx, None, None).await?;
    Ok(())
}
