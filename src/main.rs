use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use crate::data::Data;
use crate::record::Record;

pub mod data;
pub mod record;

fn main() -> std::io::Result<()> {
    // let file_path = PathBuf::from_str("wiki-topcats.txt").unwrap();
    let file_path = PathBuf::from_str("test.txt").unwrap();
    let file = File::open(file_path)?;
    let mut reader = BufReader::with_capacity(4 * 1024 * 1024, file); // 1 MiB buffer

    let mut line = String::new();
    let mut record: Record;
    let mut data = Data::new();

    while reader.read_line(&mut line)? > 0 {
        record = line.parse().unwrap();
        data.insert_record(&record);
        println!("Record inserted: {:?}", record);

        line.clear();
    }

    Ok(())
}
