#![allow(dead_code)]

use anyhow::anyhow;
use grid::Grid;

pub enum CellColor {
    Tetromino(TetrominoVariant),
    Garbage,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Rotation {
    pub piece: TetrominoVariant,
    pub from: RotationState,
    pub to: RotationState,
}

#[repr(i8)]
pub enum Direction {
    CW = 1,
    CCW = 3,
    /// Represents a 180 degree rotation
    Flip = 2,
}

#[repr(i8)]
#[derive(PartialEq, Eq, Hash)]
pub enum RotationState {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

impl From<i8> for RotationState {
    fn from(n: i8) -> Self {
        unsafe { std::mem::transmute(n % 4) }
    }
}

pub struct Tetromino {
    variant: TetrominoVariant,
    rotation_state: RotationState,
    position: (usize, usize),
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
