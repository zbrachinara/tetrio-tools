use std::fs::OpenOptions;
use std::io::Write;

use bsr_tools::action::Action;
use tetrio_replay::reconstruct;

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

#[test]
fn zbrachi_standard() {
    if let Err(action_list) = reconstruct_from_bytes(
        include_bytes!("samples/zbrachi_standard.ttr"),
        "zbrachi_standard.out",
    ) {
        println!(
            "Test zbrachi_standard could not open the output file was writing, output going to stderr instead"
        );
        eprintln!("zbrachi custom game actions: {action_list:?}");
    }
}

#[test]
fn reconstruct_40l() {
    if let Err(action_list) = reconstruct_from_bytes(include_bytes!("samples/40l.ttr"), "40l.out") {
        println!(
            "Test 40l could not open the output file was writing, output going to stderr instead"
        );
        eprintln!("40l actions: {action_list:?}");
    }
}
