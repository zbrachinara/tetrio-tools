#![allow(dead_code)]

use grid::Grid;

pub enum CellColor {
    Tetromino(TetrominoVariant),
    Garbage,
}

#[repr(i8)]
pub enum Direction {
    CW = 1,
    CCW = -1,
}

#[repr(i8)]
pub enum RotationState {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,   
}

pub struct Tetromino {
    variant: TetrominoVariant,
    rotation_state: RotationState,
    position: (usize, usize),
}

#[rustfmt::skip]
pub enum TetrominoVariant {
    L, J, T, Z, S, O, I
}

pub struct Board {
    cells: Grid<Option<CellColor>>,
    active: Tetromino,
}

pub struct Change {
    location: (usize, usize),
    to: Option<CellColor>,
}
