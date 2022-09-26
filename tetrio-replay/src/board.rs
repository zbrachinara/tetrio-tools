#![allow(dead_code)]

mod kick_table;

use std::{iter, ops::Add};

use grid::Grid;
use tap::Tap;

use crate::board::kick_table::Positions;

#[derive(Clone, Debug)]
pub enum Cell {
    Tetromino(MinoVariant),
    Garbage,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Rotation {
    pub piece: MinoVariant,
    pub from: Direction,
    pub to: Direction,
}

#[derive(Copy, Clone)]
#[repr(i8)]
pub enum Spin {
    CW = 1,
    CCW = 3,
    /// Represents a 180 degree rotation
    Flip = 2,
}

#[repr(i8)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl From<i8> for Direction {
    fn from(n: i8) -> Self {
        unsafe { std::mem::transmute(n % 4) }
    }
}

impl Add<Spin> for Direction {
    type Output = Direction;

    fn add(self, rhs: Spin) -> Self::Output {
        (self as i8 + rhs as i8).into()
    }
}

#[derive(Clone)]
pub struct Mino {
    variant: MinoVariant,
    rotation_state: Direction,
    position: (usize, usize),
}

impl Mino {
    pub fn rotation(&self, at: Spin) -> Rotation {
        Rotation {
            piece: self.variant,
            from: self.rotation_state,
            to: self.rotation_state + at,
        }
    }

    pub fn rotate(&self, at: Spin) -> Self {
        self.clone().tap_mut(|tet| {
            tet.rotation_state = tet.rotation_state + at;
        })
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MinoVariant {
    L, J, T, Z, S, O, I
}

pub struct Board {
    cells: Grid<Option<Cell>>,
    active: Mino,
}

impl Board {
    /// Attempts to rotate the active tetromino on the board. Returns true if successful,
    /// false otherwise.
    ///
    /// For now, assumes SRS+
    fn rotate_active(&mut self, direction: Spin) -> bool {
        let rotated = self.active.rotate(direction);
        let rotation = self.active.rotation(direction);

        let true_rotation = Positions::tetromino(rotated.clone());
        let kicks = kick_table::SRS_PLUS_KICK_TABLE.get(&rotation).unwrap();

        let accepted_kick = iter::once(&(0, 0)).chain(kicks.iter()).find(|offset| {
            let testing = true_rotation.clone() + **offset;
            self.test_empty(&testing)
        });

        accepted_kick
            .inspect(|(x, y)| {
                self.active = rotated.tap_mut(
                    |Mino {
                         position: (tet_x, tet_y),
                         ..
                     }| {
                        *tet_x = tet_x.wrapping_add_signed(*x as isize);
                        *tet_y = tet_y.wrapping_add_signed(*y as isize);
                    },
                )
            })
            .is_some()
    }

    /// Tests whether or not the positions passed in are empty (i.e. they are available for a
    /// tetromino to rotate into)
    ///
    /// Keep in mind that this also tests for the buffer which exists above the region in which
    /// it it legal to place tetrominos
    fn test_empty<const N: usize>(&self, positions: &Positions<N>) -> bool {
        positions.iter().all(|(x, y)| {
            // check the position is within the bounds of the board
            (*x >= 0 && *y >= 0) &&
            // and that the cell at that position is empty on the board
                self.cell(*x as usize, *y as usize)
                    .map(|u| u.is_none())
                    == Some(true)
        })
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Option<Cell>> {
        self.cells.get(y, x)
    }
}

#[cfg(test)]
mod test {
    use grid::{grid, Grid};

    use crate::board::Cell;

    use super::{Board, Spin, Direction, Mino, MinoVariant};

    #[test]
    fn test_rotations() {
        let mut board = Board {
            active: Mino {
                variant: super::MinoVariant::T,
                rotation_state: super::Direction::Down,
                position: (5, 20),
            },
            cells: Grid::init(40, 10, None),
        };

        board.rotate_active(Spin::CW);
    }

    #[test]
    fn test_t_kicks() {
        const GB: Option<Cell> = Some(Cell::Garbage);

        let mut tki_board = Board {
            active: Mino {
                variant: MinoVariant::T,
                rotation_state: Direction::Right,
                position: (1, 2),
            },
            cells: grid![
                [GB, GB, None, GB, GB, GB, GB, GB, GB, GB]
                [GB, None, None, None, GB, GB, GB, GB, GB, GB]
                [GB, None, None, GB, GB, GB, GB, None, None, None]
                [None, None, None, GB, GB, GB, None, None, None, None]
                [None, None, None, None, None, None, None, None, None, None]
                [None, None, None, None, None, None, None, None, None, None]
                [None, None, None, None, None, None, None, None, None, None]
            ],
        };

        tki_board.rotate_active(Spin::CW);
        assert_eq!(tki_board.active.position, (2, 1));
    }
}
