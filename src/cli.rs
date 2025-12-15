use clap::Parser;
use std::path::PathBuf;
use tracing::level_filters::LevelFilter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to configuration file
    #[clap(short, long, value_parser, default_value = "monitor.toml")]
    pub config: PathBuf,

    /// Path to base directory for Pico CSS. Will take precedence over configuration file value
    #[clap(short, long, value_parser)]
    pub pico_css_base_path: Option<String>,

    /// Set how verbose logging level should be
    #[clap(short, long, value_enum)]
    pub verbosity: Option<LevelFilter>,

    /// Download Pico CSS archive and exit
    #[clap(long, conflicts_with = "config")]
    pub download_pico_css: bool,
}
