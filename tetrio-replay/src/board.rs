use grid::Grid;

pub enum CellColor {
    Tetromino(TetrominoVariant),
    Garbage,
}

#[repr(u8)]
pub enum RotationState {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,   
}

pub struct Tetromino {
    variant: TetrominoVariant,
    rotation_state: RotationState,
}

#[rustfmt::skip]
pub enum TetrominoVariant {
    L, J, T, Z, S, O, I
}

pub struct Board {
    cells: Grid<Option<CellColor>>,
}

pub struct Change {
    location: (usize, usize),
    to: Option<CellColor>,
}
