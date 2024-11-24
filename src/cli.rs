use clap::Parser;
use std::path::PathBuf;
use tracing::level_filters::LevelFilter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to configuration file
    #[clap(short, long, value_parser)]
    pub config: PathBuf,

    /// Set how verbose logging level should be
    #[clap(short, long, value_enum)]
    pub verbosity: Option<LevelFilter>,
}
