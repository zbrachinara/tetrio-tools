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

/// Holds the state of the tetrio board, which can be updated through the issuing of commands.
/// The board does not keep a buffer of the commands, but transforms the commands it is issued
/// into actions, which are returned immediately. This is used to externally build up a sequence
/// of actions while having a copy of the state with which to determine which actions are possible
/// (such as the validity of rotation) and which actions would happen as a consequence of previous
/// actions (such as line clears).
pub struct Board {
    pub cells: VecGrid<Cell>,
    pub queue: PieceQueue,
    pub active: Mino,
    hold: Option<Mino>, //TODO: Combine hold fields
    hold_available: bool,
}

impl Board {
    /// Creates a new board from a PRNG seed and a board that may be filled with some cells.
    ///
    /// The format of the matrix is the same as the format found in ttr and ttrm files -- that is,
    /// as a two-dimensional matrix.
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
            hold: None,
            hold_available: true,
        }
    }

    /// Expends and returns the currently active piece, replacing it with the next piece in the
    /// queue. Meant for internal use, such as during holding/after hard dropping.
    fn cycle_piece(&mut self) -> Mino {
        std::mem::replace(&mut self.active, Mino::from(self.queue.pop()))
    }

    /// Shifts the active tetromino by the given amount of cells.
    fn shift(&mut self, cells: i8) -> Option<Action> {
        unimplemented!()
    }

    /// Holds a piece if that is possible.
    pub fn hold(&mut self) -> Option<Action> {
        let ret = self.hold_available.then(|| {
            match self.hold {
                Some(ref mut held) => std::mem::swap(&mut self.active, held),
                None => self.hold = Some(self.cycle_piece()),
            }
            Action::Hold
        });

        self.hold_available = false;

        ret
    }

    /// Drops the active tetromino into the columns that it takes up, then checks if the position
    /// that it was dropped to was legal (i.e. the tetromino is partially below the 20-cell line).
    fn drop_active(&mut self) -> Vec<Action> {
        self.hold_available = true;

        let dropping = self.cycle_piece();
        let height = dropping.center.1;

        let checkable_positions = (0..=height)
            .rev()
            .map(|y| {
                dropping.clone().tap_mut(|a| {
                    let (x, _) = a.center;
                    a.center = (x, y);
                })
            })
            .peekable();

        // TODO: Test and modify returned actions for line clears

        let most_valid = checkable_positions
            .take_while(|mino| self.test_empty(&mino.position()))
            .last()
            .unwrap();

        // TODO: Then check if the position is legal and return from there

        unimplemented!()
    }

    /// Attempts to rotate the active tetromino on the board.
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
                         center: (tet_x, tet_y),
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

    /// Tests whether or not the current position of the piece, if placed, will end the game
    /// (i.e. if it is not placed at least partially below 20 lines -- other board heights will be
    /// implemented later). Does not test whether the next piece is allowed to spawn after
    /// placement of this piece.
    fn test_legal<const N: usize>(&self, positions: &Positions<N>) -> bool {
        positions.iter().any(|(_, y)| *y < 20)
    }

    /// Tests whether or not the positions passed in intersect with the wall or other filled cells
    /// (which may, for example, imply they are available for a tetromino to rotate into) Also
    /// tests within the buffer above the region in which it it legal to place tetrominos
    fn test_empty<const N: usize>(&self, positions: &Positions<N>) -> bool {
        positions.iter().all(|(x, y)| {
            // check the position is within the lower bounds of the board
            (*x >= 0 && *y >= 0) &&
            // and that the cell at that position is empty on the board (and within the upper
            // bounds of the board)
                self.cell(*x, *y)
                    .map(|u| u.is_empty())
                    == Some(true)
        })
    }

    /// Gets the cell at the given position (x: column, y: row)
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
                direction: Direction::Down,
                center: (5, 20),
            },
            queue: PieceQueue::meaningless(),
            // cells: Grid::init(40, 10, Cell::None),
            cells: VecGrid::new_fill((Rows(40), Columns(10)), &Cell::Empty).unwrap(),
            hold: None,
            hold_available: true,
        };

        board.rotate_active(Spin::CW);
    }

    #[test]
    fn test_t_kicks() {
        const GB: Cell = Cell::Garbage;
        const NC: Cell = Cell::Empty;

        let mut tki_board = Board {
            active: Mino {
                variant: MinoVariant::T,
                direction: Direction::Right,
                center: (1, 2),
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
            hold: None,
            hold_available: true,
        };

        tki_board.rotate_active(Spin::CW);
        assert_eq!(tki_board.active.center, (2, 1));
    }
}
