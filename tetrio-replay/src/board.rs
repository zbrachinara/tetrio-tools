#![allow(dead_code)]

mod kick_table;

use std::{iter, ops::Add};

use gridly::prelude::{Column, Grid, Row};
use gridly_grids::VecGrid;
use tap::Tap;

use crate::{
    board::kick_table::Positions, data::event::Game, reconstruct::Action, rng::PieceQueue,
};

#[derive(Clone, Debug)]
pub enum Cell {
    Tetromino(MinoVariant),
    Garbage,
    None,
}

impl Cell {
    fn is_empty(&self) -> bool {
        matches!(self, Cell::None)
    }
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

impl From<MinoVariant> for Mino {
    fn from(variant: MinoVariant) -> Self {
        Self {
            variant,
            rotation_state: Direction::Up,
            position: (5, 22), //TODO: Find out if the piece actually spawns here initially
        }
    }
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
    cells: VecGrid<Cell>,
    active: Mino,
}

impl Board {
    // fn new(queue: PieceQueue, game: &Game) -> Self {

    //     let board = game.board;
    //     let cells = Grid::new(0, 0);
    //     for row in board {
    //         cells.push_row(row)
    //     }

    //     Self {
    //         cells,
    //         queue,
    //         active: queue.pop().into(),
    //     }
    // }

    /// Attempts to rotate the active tetromino on the board. Returns true if successful,
    /// false otherwise.
    ///
    /// For now, assumes SRS+
    pub fn rotate_active(&mut self, direction: Spin) -> Option<Action> {
        let rotated = self.active.rotate(direction);
        let rotation = self.active.rotation(direction);

        let true_rotation = Positions::tetromino(&rotated);
        let kicks = kick_table::SRS_PLUS.get(&rotation).unwrap(); // where SRS+ assumed

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
            .map(|_| Action::Reposition {
                piece: self.active.clone(),
            })
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
                self.cell(*x, *y)
                    .map(|u| u.is_empty())
                    == Some(true)
        })
    }

    fn cell(&self, x: isize, y: isize) -> Option<&Cell> {
        self.cells.get((Column(x), Row(y))).ok()
    }
}

#[cfg(test)]
mod test {

    use gridly::vector::{Columns, Rows};
    use gridly_grids::VecGrid;

    use crate::board::Cell;

    use super::{Board, Direction, Mino, MinoVariant, Spin};

    #[test]
    fn test_rotations() {
        let mut board = Board {
            active: Mino {
                variant: super::MinoVariant::T,
                rotation_state: super::Direction::Down,
                position: (5, 20),
            },
            // cells: Grid::init(40, 10, Cell::None),
            cells: VecGrid::new_fill((Rows(40), Columns(10)), &Cell::None).unwrap(),
        };

        board.rotate_active(Spin::CW);
    }

    #[test]
    fn test_t_kicks() {
        const GB: Cell = Cell::Garbage;
        const NC: Cell = Cell::None;

        let mut tki_board = Board {
            active: Mino {
                variant: MinoVariant::T,
                rotation_state: Direction::Right,
                position: (1, 2),
            },
            cells: VecGrid::new_from_rows(vec![
                [GB, GB, NC, GB, GB, GB, GB, GB, GB, GB],
                [GB, NC, NC, NC, GB, GB, GB, GB, GB, GB],
                [GB, NC, NC, GB, GB, GB, GB, NC, NC, NC],
                [NC, NC, NC, GB, GB, GB, NC, NC, NC, NC],
                [NC, NC, NC, NC, NC, NC, NC, NC, NC, NC],
                [NC, NC, NC, NC, NC, NC, NC, NC, NC, NC],
                [NC, NC, NC, NC, NC, NC, NC, NC, NC, NC],
            ]).unwrap(),
        };

        tki_board.rotate_active(Spin::CW);
        assert_eq!(tki_board.active.position, (2, 1));
    }
}
