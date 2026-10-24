use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

use crate::cli::Cli;
use crate::data::Data;
use crate::record::Record;

mod cli;
pub mod data;
pub mod record;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_max_level(cli.log_level)
        .init();

    let file_path = cli.data_file_path;

    let file = File::open(file_path)?;
    let mut reader = BufReader::with_capacity(4 * 1024 * 1024, file); // 1 MiB buffer

    let mut line = String::new();
    let mut record: Record;
    let mut data = Data::new();

    while reader.read_line(&mut line)? > 0 {
        record = line.parse().unwrap();
        data.insert_record(&record);
        tracing::info!("Record inserted: {:?}", record);

        line.clear();
    }

    Ok(())
}
