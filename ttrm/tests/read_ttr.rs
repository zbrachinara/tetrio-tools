#[test]
fn blitz_ttr() {
    println!(
        "{:?}",
        serde_json::from_slice::<ttrm::Ttr>(include_bytes!("../../samples/blitz.ttr")).unwrap()
    );
}

#[test]
fn forty_line_ttr() {
    println!(
        "{:?}",
        serde_json::from_slice::<ttrm::Ttr>(include_bytes!("../../samples/_40l.ttr")).unwrap()
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
