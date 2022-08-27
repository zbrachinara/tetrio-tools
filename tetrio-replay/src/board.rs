use grid::Grid;

pub enum CellColor {
    L,
    J,
    T,
    Z,
    S,
    O,
    I,
    Garbage,
}

pub struct Board {
    cells: Grid<Option<CellColor>>,
}

pub struct Change {
    location: (usize, usize),
    to: Option<CellColor>,
}