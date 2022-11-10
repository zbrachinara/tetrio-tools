#![allow(dead_code)]

use std::iter;

use gridly::prelude::{Column, Grid, GridMut, Row, GridBounds};
use itertools::Itertools;
use tap::Tap;

use crate::rng::PieceQueue;
use bsr_tools::{
    action::Action,
    kick_table::{self, Positions},
    tetromino::{Cell, Mino, Spin},
};

use self::storage::BoardStorage;

mod storage;

/// Holds the state of the tetrio board, which can be updated through the issuing of commands.
/// The board does not keep a buffer of the commands, but transforms the commands it is issued
/// into actions, which are returned immediately. This is used to externally build up a sequence
/// of actions while having a copy of the state with which to determine which actions are possible
/// (such as the validity of rotation) and which actions would happen as a consequence of previous
/// actions (such as line clears).
pub struct Board {
    pub cells: BoardStorage<Cell>,
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
        let cells = BoardStorage::new_from_rows_unchecked(
            game.iter()
                .map(|row| row.iter().map(|elem| Cell::from(*elem)).collect_vec())
                .rev()
                .collect_vec(),
        );

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

    /// Drops the active tetromino into the lowest possible position within the columns it takes up.
    pub fn drop_active(&mut self) -> Vec<Action> {
        self.hold_available = true;

        let dropping = self.cycle_piece();
        let variant: Cell = dropping.variant.into();
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

        let dropped = checkable_positions
            .map(|mino| mino.position())
            .take_while(|position| self.test_empty(&position))
            .last()
            .unwrap(); // valid because the mino's current position is guaranteed valid

        // populate the cells which have been dropped into
        dropped.iter().for_each(|position| {
            *self.cell_mut(position.0, position.1).unwrap() = variant.clone();
        });

        let mut height_offset = 0;

        //TODO: Propogate the line clears to the board representation
        dropped
            .lowest_first()
            .iter()
            .peekable()
            .batching(|it| {
                let (pos, height) = it.next()?;

                // if we're not dealing with a filled column, then report the positions of filled
                // cells
                let mut sto = self.is_filled(*height).unwrap().then(|| {
                    height_offset -= 1;
                    vec![Action::Cell {
                        position: (*pos as u8, (*height + height_offset) as u8),
                        kind: variant.clone(),
                    }]
                });

                // keep taking elements until the next height
                while let Some((_, h)) = it.peek() {
                    if h != height {
                        break;
                    } else {
                        let (pos, height) = it.next().unwrap();
                        sto.iter_mut().for_each(|v| {
                            v.push(Action::Cell {
                                position: (*pos as u8, (*height + height_offset) as u8),
                                kind: variant.clone(),
                            })
                        });
                    }
                }

                Some(sto.unwrap_or(vec![Action::LineClear {
                    row: (*height + height_offset) as u8,
                }]))
            })
            .flatten()
            .collect()
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

    /// Tests if a row is filled, and therefore should be cleared. Returns `None` if the given
    /// row is invalid
    fn is_filled(&self, row: isize) -> Option<bool> {
        self.cells
            .row(row)
            .ok()
            .map(|row| !row.iter().any(|cell| cell.is_empty()))
    }

    /// Tests whether or not the current position of the piece, if placed, will end the game
    /// (i.e. if it is not placed at least partially below 20 lines -- other board heights will be
    /// implemented later). Does not test whether the next piece is allowed to spawn after
    /// placement of this piece.
    fn test_legal<const N: usize>(&self, positions: &Positions<N>) -> bool {
        positions.iter().any(|(_, y)| *y < 20 && *y >= 0)
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

    fn cell_mut(&mut self, x: isize, y: isize) -> Option<&mut Cell> {
        self.cells.get_mut((Column(x), Row(y))).ok()
    }
}

#[cfg(test)]
mod test {

    use gridly::prelude::*;
    use itertools::Itertools;

    use super::{storage::BoardStorage, Board};
    use crate::{board::Cell, rng::PieceQueue};

    use bsr_tools::tetromino::{Direction, Mino, MinoVariant, Spin};

    /// Takes a map exported from [https://tetrio.team2xh.net/?t=editor] and converts it to
    /// a [BoardStorage]
    fn board_from_string(s: &str) -> BoardStorage<Cell> {
        use MinoVariant::*;
        let cells_chunks = s
            .chars()
            .filter_map(|ch| match ch {
                '_' => Some(Cell::Empty),
                'z' => Some(Cell::Tetromino(Z)),
                'l' => Some(Cell::Tetromino(L)),
                'o' => Some(Cell::Tetromino(O)),
                's' => Some(Cell::Tetromino(S)),
                'i' => Some(Cell::Tetromino(I)),
                'j' => Some(Cell::Tetromino(J)),
                't' => Some(Cell::Tetromino(T)),
                '#' => Some(Cell::Garbage),
                _ => None,
            })
            .chunks(10);
        let mut cells = cells_chunks
            .into_iter()
            .map(|r| r.collect_vec())
            .collect_vec();

        cells.reverse();

        BoardStorage::new_from_rows_unchecked(cells)
    }

    /// Creates a board containing nothing
    fn empty_board() -> BoardStorage<Cell> {
        board_from_string("________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________")
    }

    #[test]
    fn test_rotations() {
        let mut board = Board {
            active: Mino {
                variant: MinoVariant::T,
                direction: Direction::Down,
                center: (5, 20),
            },
            queue: PieceQueue::meaningless(),
            cells: empty_board(),
            hold: None,
            hold_available: true,
        };

        board.rotate_active(Spin::CW);
    }

    #[test]
    fn test_t_kicks() {
        let mut tki_board = Board {
            active: Mino {
                variant: MinoVariant::T,
                direction: Direction::Right,
                center: (1, 2),
            },
            queue: PieceQueue::meaningless(),
            // the flat-top tki made with garbage cells built with tspin on the left
            cells: board_from_string("___________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________###____#__####___#___########_#######"),
            hold: None,
            hold_available: true,
        };

        tki_board.rotate_active(Spin::CW);
        assert_eq!(tki_board.active.center, (2, 1));

        let mut tst_board = Board {
            cells: board_from_string("__________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________#_______________####_#########__########_#####"),
            queue: PieceQueue::meaningless(),
            active: Mino {
                variant: MinoVariant::T,
                direction: Direction::Up,
                center: (5, 3)
            },
            hold: None,
            hold_available: true,
        };

        tst_board.rotate_active(Spin::CW);
        assert_eq!(tst_board.active.center, (4, 1))
    }

    #[test]
    fn test_drops() {
        {
            let board_initial = board_from_string("________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________#__########___#########__#____###___#__####______###_____#_##_#_____#_#_#_#_____#_###_#####___######____#####_###_####_#");
            let board_final = board_from_string("________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________#__########___#########__#____###___#__####______###_____#_##_#jjj__#_#_#_#j____#_###_#####___######____#####_###_####_#");

            let mut b = Board {
                cells: board_initial,
                queue: PieceQueue::meaningless(),
                active: Mino {
                    variant: MinoVariant::J,
                    direction: Direction::Down,
                    center: (4, 7),
                },
                hold: None,
                hold_available: true,
            };

            b.drop_active();

            b.cells
                .rows()
                .iter()
                .map(|c| c.iter())
                .flatten()
                .zip(board_final.rows().iter().map(|c| c.iter()).flatten())
                .for_each(|(a1, a2)| {
                    assert_eq!(
                        a1,
                        a2,
                        "final state:\n{}",
                        b.cells.display_with(|u| u.clone())
                    )
                });
        }
    }
}
