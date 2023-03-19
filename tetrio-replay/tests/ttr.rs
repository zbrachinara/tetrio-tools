use std::fs::OpenOptions;
use std::io::Write;

use tetrio_replay::reconstruct;
use viewtris::action::Action;

fn reconstruct_from_bytes(bytes: &[u8], write_to: &str) -> Result<(), Vec<Action>> {
    let ttr = serde_json::from_slice::<ttrm::Ttr>(bytes)
        .expect("TTR parsing is not working correctly, check tests in ttr crate");

    let action_list = reconstruct(&ttr.data.events).expect("Reconstruction step failed");

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

macro_rules! ttr_test {
    ($name:tt) => {
        #[test]
        fn $name() {
            if let Err(action_list) = reconstruct_from_bytes(
                include_bytes!(concat!("../../samples/", stringify!($name), ".ttr")),
                concat!(stringify!($name), ".out"),
            ) {
                println!(concat!(
                    "Test",
                    stringify!($name),
                    " could not open the output file was writing, output going to stderr instead"
                ));
                eprintln!(concat!(stringify!($name), " actions: {:?}"), action_list);
            }
        }
    };
}

ttr_test!(zbrachi_standard);

ttr_test!(hahahaki);

ttr_test!(garbage);

ttr_test!(_40l);
