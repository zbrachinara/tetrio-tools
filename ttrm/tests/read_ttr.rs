#[test]
fn forty_line_ttr() {
    println!(
        "{:?}",
        serde_json::from_slice::<ttrm::Ttr>(include_bytes!("40l.ttr")).unwrap()
    );
}

#[test]
fn custom_board_ttr() {
    println!(
        "{:?}",
        serde_json::from_slice::<ttrm::Ttr>(include_bytes!("large_board.ttr")).unwrap()
    );
}