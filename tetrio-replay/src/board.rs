use grid::Grid;

pub enum CellColor {
    Tetromino(Tetromino),
    Garbage,
}

#[rustfmt::skip]
pub enum Tetromino {
    L, J, T, Z, S, O, I
}

pub struct Board {
    cells: Grid<Option<CellColor>>,
}

pub struct Change {
    location: (usize, usize),
    to: Option<CellColor>,
}
