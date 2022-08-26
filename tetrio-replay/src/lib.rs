#![feature(let_else)]

use std::{fs::read_to_string, path::Path};

pub use data::TTRM;
use thiserror::Error;

pub mod data;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Serde error: {0}")]
    Serde(serde_json::Error),
    #[error("File error: {0}")]
    Fs(std::io::Error)
}

impl From<serde_json::Error> for Error {
    fn from(s: serde_json::Error) -> Self {
        Self::Serde(s)
    }
}

impl From<std::io::Error> for Error {
    fn from(s: std::io::Error) -> Self {
        Self::Fs(s)
    }
}

pub fn parse_replay(path: impl AsRef<Path>) -> Result<TTRM, Error> {
    Ok(serde_json::from_str(&read_to_string(path)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_replay_test() {
        println!("{:?}", parse_replay("src/HBSQabUhSS.ttrm"));
    }
}
