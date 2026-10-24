use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::{data::Data, record::Record};

#[derive(Debug, Error)]
pub enum DataLoaderError {
    #[error("failed to open {path}")]
    Open {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to read {path}")]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

pub struct DataLoader;

impl DataLoader {
    const BUFFER_CAPACITY: usize = 4_000_000; // 4MB

    pub fn load_from_path(path: &Path) -> Result<Data, DataLoaderError> {
        let file = File::open(path).map_err(|source| DataLoaderError::Open {
            path: path.to_owned(),
            source,
        })?;
        let mut reader = BufReader::with_capacity(Self::BUFFER_CAPACITY, file);

        let mut data = Data::new();
        let mut record: Record;
        let mut line = String::new();
        let mut i = 0;

        while reader
            .read_line(&mut line)
            .map_err(|source| DataLoaderError::Read {
                path: path.to_owned(),
                source,
            })?
            > 0
        {
            record = line.parse().unwrap();
            data.insert_record(&record);

            tracing::trace!("Parsed: {:?}", record);
            if i % 100 == 0 {
                tracing::debug!("Parsed {i} records");
            }

            line.clear();
            i += 1;
        }
        tracing::info!("Done. Parsed {i} records");

        Ok(data)
    }
}
