#![allow(non_snake_case)]

use std::fs::OpenOptions;

use std::io::Write;
use tetrio_replay::reconstruct;
use ttrm::event::Event;
use viewtris::action::Action;

fn reconstruct_from_events(events: &[Event], write_to: &str) -> Result<(), Vec<Action>> {
    let action_list = reconstruct(events).expect("Reconstruction step failed");

    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(write_to)
        .and_then(|mut out_file| {
            action_list
                .clone()
                .into_iter()
                .try_for_each(|action| writeln!(out_file, "{action:?}"))
        })
        .map_err(|_| action_list)
}

macro_rules! ttrm_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            let ttr = serde_json::from_slice::<ttrm::Ttrm>(include_bytes!(concat!(
                "../../samples/",
                stringify!($name),
                ".ttrm"
            )))
            .expect("TTRM parsing is not working correctly, check tests in ttrm crate");

            for (i, data) in ttr.data.iter().enumerate() {
                for (j, replay) in data.replays.iter().enumerate() {
                    let write_to = format!(concat!(stringify!($name), "_{}_{}"), i, j);

                    if let Err(action_list) = reconstruct_from_events(&replay.events, &write_to) {
                        println!(concat!(
                            "Test ",
                            stringify!($name),
                            " could not open the output file was writing, output going to stderr instead"
                        ));
                        eprintln!(concat!(stringify!($name), " actions_{}_{}: {:?}"), i, j, action_list);
                    }
                }
            }

        }
    };
}

ttrm_test!(HBSQabUhSS);
