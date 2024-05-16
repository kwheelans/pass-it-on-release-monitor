use std::path::PathBuf;
use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to pass-it-on client configuration file
    #[clap(short, long, value_parser)]
    pub client_config: Option<PathBuf>,

    /// Path to release monitor configuration
    #[clap(short, long, value_parser)]
    pub monitor_config: Option<PathBuf>,
    
    /// Set how verbose logging level should be
    #[clap(short, long, value_enum, default_value = "info")]
    pub verbosity: LevelFilter,

}