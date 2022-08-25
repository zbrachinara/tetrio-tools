#![feature(let_else)]

use std::{fs::read_to_string, path::Path};

use data::TTRM;

mod data;

pub fn parse_replay(path: impl AsRef<Path>) {
    let obj: TTRM = serde_json::from_str(&read_to_string(path).unwrap()).unwrap();

    // println!("{:?}", obj.data[0].replays[0].events[10]);

    // let Value::Object(ref data) = obj.data[0].replays[0] else {panic!()};

    // data.iter().for_each(|(k, v)| {
    //     match v {
    //         Value::Array(_) => println!("{k}: array..."),
    //         Value::Object(_) => println!("{k}: object..."),
    //         _ => println!("{k}: {v}")
    //     }
    // });
    // obj.data.iter().for_each(|set| {
    //     set.replays
    //         .iter()
    //         .for_each(|replay| replay.events.iter().for_each(
    //             |event| {
    //                 match event.data {
    //                     _ => ()
    //                 }
    //             }
    //         ))
    // });

    println!("{:#?}", obj.data[0].replays)
    // println!("{}", obj.data.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_replay_test() {
        parse_replay("src/HBSQabUhSS.ttrm")
    }
}
