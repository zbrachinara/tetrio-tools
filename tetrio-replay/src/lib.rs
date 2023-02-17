#![feature(drain_filter)]

use anyhow::Result;
use ttrm::Ttrm;

mod board;
mod reconstruct;
mod rng;

pub fn parse_replay<'a>(content: &'a str) -> Result<Ttrm<'a>> {
    Ok(serde_json::from_str(content)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn parse_replay_test() {
        println!(
            "{:#?}",
            parse_replay(&read_to_string("src/HBSQabUhSS.ttrm").unwrap()).unwrap()
        );
    }
}
