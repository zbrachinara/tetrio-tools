#![feature(let_else)]

use std::{fs::read_to_string, path::Path};

pub use data::TTRM;

pub mod data;

pub fn parse_replay(path: impl AsRef<Path>) -> TTRM {
    serde_json::from_str(&read_to_string(path).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_replay_test() {
        parse_replay("src/HBSQabUhSS.ttrm");
    }
}
