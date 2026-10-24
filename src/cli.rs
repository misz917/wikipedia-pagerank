use std::path::PathBuf;

use clap::Parser;
use tracing::Level;

#[derive(Debug, Parser)]
pub struct Cli {
    pub data_file_path: PathBuf,

    #[arg(long, default_value_t = Level::INFO)]
    pub log_level: Level,
}
