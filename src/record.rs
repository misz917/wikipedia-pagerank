use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

use crate::record::RecordParsingError::FailedToSplit;

#[derive(Debug)]
pub struct Record([u32; 2]);

#[derive(Error, Debug)]
pub enum RecordParsingError {
    #[error("expected two space-separated values")]
    FailedToSplit,
    #[error("failed to parse first value: {0}")]
    FailedToParseA(#[source] ParseIntError),
    #[error("failed to parse second value: {0}")]
    FailedToParseB(#[source] ParseIntError),
}

impl FromStr for Record {
    type Err = RecordParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_string = s.split_whitespace();

        let (Some(a), Some(b)) = (split_string.next(), split_string.next()) else {
            return Err(FailedToSplit);
        };

        let u32a: u32 = a
            .trim()
            .parse()
            .map_err(RecordParsingError::FailedToParseA)?;

        let u32b: u32 = b
            .trim()
            .parse()
            .map_err(RecordParsingError::FailedToParseB)?;

        Ok(Record([u32a, u32b]))
    }
}

impl Record {
    #[inline]
    pub fn links_from(&self) -> u32 {
        self.0[0]
    }

    #[inline]
    pub fn links_to(&self) -> u32 {
        self.0[1]
    }
}
