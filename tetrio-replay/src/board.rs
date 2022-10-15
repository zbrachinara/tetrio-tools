#![allow(dead_code)]

use std::iter;

use gridly::prelude::{Column, Grid, Row};
use gridly_grids::VecGrid;
use tap::Tap;

use crate::rng::PieceQueue;
use bsr_tools::{
    action::Action,
    kick_table::{self, Positions},
    tetromino::{Cell, Mino, Spin},
};

pub struct Board {
    pub cells: VecGrid<Cell>,
    pub queue: PieceQueue,
    pub active: Mino,
}

impl Board {
    pub fn new(piece_seed: u64, game: &Vec<Vec<Option<&str>>>) -> Self {
        let mut queue = PieceQueue::seeded(piece_seed, 5);
        let cells = VecGrid::new_from_rows(
            game.iter()
                .map(|row| row.iter().map(|elem| Cell::from(*elem)))
                .rev(),
        )
        .unwrap();

        let active = queue.pop().into();

        Self {
            cells,
            queue,
            active,
        }
    }

    /// Drops the active tetromino into the columns that it takes up, then checks if the position
    /// that it was dropped to was legal (i.e. one cell landed below 20 lines)
    fn drop_active(&mut self) -> Vec<Action> {
        unimplemented!()
    }

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

    use super::Board;
    use crate::{board::Cell, rng::PieceQueue};

    use bsr_tools::tetromino::{Direction, Mino, MinoVariant, Spin};

    #[test]
    fn test_rotations() {
        let mut board = Board {
            active: Mino {
                variant: MinoVariant::T,
                rotation_state: Direction::Down,
                position: (5, 20),
            },
            queue: PieceQueue::meaningless(),
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
            queue: PieceQueue::meaningless(),
            cells: VecGrid::new_from_rows(vec![
                [GB, GB, NC, GB, GB, GB, GB, GB, GB, GB],
                [GB, NC, NC, NC, GB, GB, GB, GB, GB, GB],
                [GB, NC, NC, GB, GB, GB, GB, NC, NC, NC],
                [NC, NC, NC, GB, GB, GB, NC, NC, NC, NC],
                [NC, NC, NC, NC, NC, NC, NC, NC, NC, NC],
                [NC, NC, NC, NC, NC, NC, NC, NC, NC, NC],
                [NC, NC, NC, NC, NC, NC, NC, NC, NC, NC],
            ])
            .unwrap(),
        };

        tki_board.rotate_active(Spin::CW);
        assert_eq!(tki_board.active.position, (2, 1));
    }
}
