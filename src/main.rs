mod error;
mod models;
mod cli;
mod configuration;

use std::process::ExitCode;
use clap::Parser;
use log::{debug, error, info, LevelFilter};
use crate::cli::CliArgs;
use crate::error::Error;
use crate::models::rancher_channel_server::get_channels;


const LOG_TARGET: &str = "pass_it_on_release_monitor";
#[tokio::main]
async fn main() -> ExitCode {
    let args = CliArgs::parse();
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
    info!("Begin monitoring");
    let search = "stable"; //TODO: get from config
    let channels = get_channels(search).await?;
    debug!("Received RKE2 Channels");
    let search = "stable";
    
    match channels.data.into_iter().find(|c| c.id.eq_ignore_ascii_case(search)) {
        Some(channel) => {
            debug!("Name: {} Version: {}", channel.name, channel.latest);
            Ok(())
        }
        None => {
            Err(Error::NoChannelFound(search.to_string()))
        }
    }
}
