use tetrio_replay::reconstruct;

#[test]
fn reconstruct_40l() {
    let ttr = serde_json::from_slice::<ttrm::Ttr>(include_bytes!("samples/40l.ttr"))
        .expect("TTR parsing is not working correctly, check tests in ttr crate");

    let action_list = reconstruct(&ttr.data.events).expect("Reconstruction step failed");

    println!("{action_list:?}")
}
