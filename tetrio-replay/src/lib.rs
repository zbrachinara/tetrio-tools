#![feature(result_option_inspect)]
#![feature(mixed_integer_ops)]

use anyhow::Result;
pub use data::TTRM;
rgo
mod board;
pub mod data;
mod rng;
mod reconstruct;

pub fn parse_replay<'a>(content: &'a str) -> Result<TTRM<'a>> {
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
