#[test]
fn forty_line_ttr() {
    println!(
        "{:?}",
        serde_json::from_slice::<ttrm::Ttr>(include_bytes!("../../samples/40l.ttr")).unwrap()
    );
}

#[test]
fn custom_board_ttr() {
    println!(
        "{:?}",
        serde_json::from_slice::<ttrm::Ttr>(include_bytes!("../../samples/large_board.ttr"))
            .unwrap()
    );
}
