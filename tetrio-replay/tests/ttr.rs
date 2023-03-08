use std::fs::OpenOptions;
use std::io::Write;

use tetrio_replay::reconstruct;

#[test]
fn reconstruct_40l() {
    let ttr = serde_json::from_slice::<ttrm::Ttr>(include_bytes!("samples/40l.ttr"))
        .expect("TTR parsing is not working correctly, check tests in ttr crate");

    let action_list = reconstruct(&ttr.data.events).expect("Reconstruction step failed");

    if OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("40l.out")
        .and_then(|mut out_file| {
            action_list
                .clone()
                .into_iter()
                .try_for_each(|action| writeln!(out_file, "{action:?}"))
        })
        .is_err()
    {
        println!(
            "Test 40l could not open the output file was writing, output going to stderr instead"
        );
        eprintln!("40l actions: {action_list:?}");
    }
}
