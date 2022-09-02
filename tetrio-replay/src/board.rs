#![allow(dead_code)]

mod kick_table;

use std::{iter, ops::Add};

use grid::Grid;
use tap::Tap;

use crate::board::kick_table::Positions;

#[derive(Clone)]
pub enum Cell {
    Tetromino(TetrominoVariant),
    Garbage,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Rotation {
    pub piece: TetrominoVariant,
    pub from: RotationState,
    pub to: RotationState,
}

#[derive(Copy, Clone)]
#[repr(i8)]
pub enum Direction {
    CW = 1,
    CCW = 3,
    /// Represents a 180 degree rotation
    Flip = 2,
}

#[repr(i8)]
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
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

impl Add<Direction> for RotationState {
    type Output = RotationState;

    fn add(self, rhs: Direction) -> Self::Output {
        (self as i8 + rhs as i8).into()
    }
}

#[derive(Clone)]
pub struct Tetromino {
    variant: TetrominoVariant,
    rotation_state: RotationState,
    position: (usize, usize),
}

impl Tetromino {
    pub fn rotation(&self, at: Direction) -> Rotation {
        Rotation {
            piece: self.variant,
            from: self.rotation_state,
            to: self.rotation_state + at,
        }
    }

    pub fn rotate(&self, at: Direction) -> Self {
        self.clone().tap_mut(|tet| {
            tet.rotation_state = tet.rotation_state + at;
        })
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum TetrominoVariant {
    L, J, T, Z, S, O, I
}

pub struct Board {
    cells: Grid<Option<Cell>>,
    active: Tetromino,
}

pub struct Change {
    location: (usize, usize),
    to: Option<Cell>,
}

impl Board {
    /// Attempts to rotate the active tetromino on the board. Returns true if successful,
    /// false otherwise.
    ///
    /// For now, assumes SRS+
    fn rotate_active(&mut self, direction: Direction) -> bool {
        let rotated = self.active.rotate(direction);
        let rotation = self.active.rotation(direction);

        let true_rotation = Positions::tetromino(rotated);
        let kicks = kick_table::SRS_PLUS_KICK_TABLE.get(&rotation).unwrap();

        let accepted_kick = iter::once(&(0, 0))
            .chain(kicks.iter())
            .find(|offset| todo!());

        println!(
            "{:?}",
            kick_table::ROTATION_TABLE.get(&(rotation.piece, rotation.to))
        );

        todo!()
    }

    /// Tests whether or not the positions passed in are empty (i.e. they are available for a
    /// tetromino to rotate into)
    /// 
    /// Keep in mind that this also tests for the buffer which exists above the region in which
    /// it it legal to place tetrominos
    fn test_empty<const N: usize>(&self, positions: &Positions<N>) -> bool {
        positions.iter().all(|(x, y)| {
            // check the position is within the bounds of the board
            (*x > 0 && *x < self.cells.cols() as isize && *y > 0 && *y < self.cells.rows() as isize)
            // and that the cell at that position is empty on the board
                && self.cells.get(*y as usize, *x as usize).is_none()
        });
        todo!()
    }
}

#[cfg(test)]
mod test {
    use grid::Grid;

    use super::{Board, Direction};

    #[test]
    fn test_rotations() {
        let mut board = Board {
            active: super::Tetromino {
                variant: super::TetrominoVariant::T,
                rotation_state: super::RotationState::Down,
                position: (5, 20),
            },
            cells: Grid::init(40, 10, None),
        };

        board.rotate_active(Direction::CW);
    }
}
