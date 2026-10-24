use clap::Parser;

use crate::cli::Cli;
use crate::data_loader::DataLoader;

mod cli;
pub mod data;
pub mod data_loader;
pub mod record;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_max_level(cli.log_level)
        .init();

    let file_path = cli.data_file_path;

    let data = DataLoader::load_from_path(&file_path).unwrap();

    Ok(())
}
