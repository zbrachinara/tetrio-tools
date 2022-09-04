#![feature(let_else)]
#![feature(result_option_inspect)]
#![feature(mixed_integer_ops)]

use std::{fs::read_to_string, path::Path};

pub use data::TTRM;
use anyhow::Result;

pub mod data;
mod reconstruct;
mod board;

pub fn parse_replay(path: impl AsRef<Path>) -> Result<TTRM> {
    Ok(serde_json::from_str(&read_to_string(path)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_replay_test() {
        println!("{:#?}", parse_replay("src/HBSQabUhSS.ttrm").unwrap());
    }
}
