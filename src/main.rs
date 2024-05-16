mod cli;
mod configuration;
mod error;
mod monitors;

use crate::cli::CliArgs;
use crate::error::Error;
use clap::Parser;
use log::{debug, error, LevelFilter};
use std::process::ExitCode;
use crate::configuration::MonitorConfigFileParser;

const LOG_TARGET: &str = "pass_it_on_release_monitor";

#[tokio::main]
async fn main() -> ExitCode {
    let args = CliArgs::parse();

    // Configure logging
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .env()
        .with_module_level(LOG_TARGET, args.verbosity)
        .with_colors(true)
        .init()
        .unwrap();

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


    for monitor in config.monitors.monitor {
        let version = monitor.check().await?;
        debug!("Got version {}", version)
    }

    Ok(())
}
