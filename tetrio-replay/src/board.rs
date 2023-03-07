#![allow(dead_code)]

use std::iter;

use gridly::prelude::{Column, Grid, GridBounds, GridMut, Row};
use itertools::{Either, Itertools};
use tap::Tap;

use crate::{
    reconstruct::{Settings, ShiftDirection, State},
    rng::PieceQueue,
};
use bsr_tools::{
    action::{Action, ActionKind},
    kick_table::Positions,
    tetromino::{Cell, Mino, MinoVariant, Spin},
};

use self::storage::BoardStorage;

mod storage;

pub enum Hold {
    Empty,
    Active(MinoVariant),
    NotActive(MinoVariant),
}

impl Hold {
    fn activate(&mut self) {
        if let Self::NotActive(x) = self {
            let p = std::mem::replace(x, MinoVariant::I); // arbitrary piece, could be anything
            *self = Self::Active(p);
        }
    }
}

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
    /// A value signifying how much time has passed since the active piece has most recently fallen
    /// (by any amount). If this piece surpasses a certain threshold, the excess is used to
    /// calculate how far this piece should fall, and/or whether or not it should lock in place
    pub gravity_state: f64,
    /// How many times the piece is able to avoid locking until it is forced to lock immediately
    lock_count: u8,
    hold: Hold,
}

impl Board {
    /// Creates a new board from a PRNG seed and a board that may be filled with some cells.
    ///
    /// The format of the matrix is the same as the format found in ttr and ttrm files -- that is,
    /// as a two-dimensional matrix.
    pub fn new(piece_seed: u64, game: &[Vec<Option<&str>>]) -> Self {
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
            gravity_state: 0.0,
            lock_count: 16,
            hold: Hold::Empty,
        }
    }

    /// Expends and returns the currently active piece, replacing it with the next piece in the
    /// queue. Meant for internal use, such as during holding/after hard dropping.
    fn cycle_piece(&mut self) -> Mino {
        std::mem::replace(&mut self.active, Mino::from(self.queue.pop()))
    }

    /// Shifts the active tetromino by the given amount of cells.
    pub fn shift(&mut self, cells: i8) -> Vec<ActionKind> {
        let shift_to = self
            .active
            .coord
            .0
            .saturating_add_signed(cells as isize)
            .clamp(0, self.cells.num_columns().0 as usize);

        let new_position =
        // ranges are inclusive since the tetromino can at least occupy its current position
        if self.active.coord.0 < shift_to {
            itertools::Either::Left(self.active.coord.0..=shift_to)
        } else {
            itertools::Either::Right((shift_to..=self.active.coord.0).rev())
        }
        .map(|x| self.active.tap_mut(|piece| piece.coord.0 = x))
        .tuple_windows()
        .find_map(|(m1, m2)| self.intersects(&m2).then_some(m1))
        .unwrap_or_else(|| self.active.tap_mut(|piece| piece.coord.0 = shift_to));

        // TODO: return None when the position has not changed
        self.reposition(new_position)
    }

    /// Holds a piece if that is possible.
    pub fn hold(&mut self) -> Option<ActionKind> {
        match self.hold {
            Hold::Empty => {
                self.hold = Hold::NotActive(self.cycle_piece().variant);
                Some(ActionKind::Hold)
            }
            Hold::Active(held) => {
                self.hold =
                    Hold::NotActive(std::mem::replace(&mut self.active, Mino::from(held)).variant);
                Some(ActionKind::Hold)
            }
            Hold::NotActive(_) => None,
        }
    }

    /// A function which handles all passive effects which happen between events, such as auto-shift
    /// and gravity/soft-drop. These effects must be handled together because each of them changes
    /// the position of the mino, which in turn changes which shifts and drops are possible.
    pub fn passive_effects(
        &mut self,
        current_subframe: u64,
        settings: &Settings,
        key_state: &State,
    ) -> Vec<Action> {
        (key_state.last_subframe..current_subframe)
            .flat_map(|subframe| {
                // TODO handle gravity acceleration
                self.gravity_state += if key_state.soft_dropping {
                    settings.sdf as f64
                } else {
                    1.
                } * settings.gravity
                    / 10.;

                let mut out = Vec::new();

                let shift_size = if settings.arr == 0 {
                    self.cells.num_columns().0 as i8
                } else if (subframe - (key_state.shift_began + settings.das)) % settings.arr == 0 {
                    1
                } else {
                    0
                };
                if shift_size != 0 {
                    // TODO: Don't emit this action if it isn't necessary
                    out.extend(match key_state.shifting {
                        ShiftDirection::None => Vec::new(),
                        ShiftDirection::Left => self.shift(-shift_size),
                        ShiftDirection::Right => self.shift(shift_size),
                    });
                }

                if self.gravity_state.trunc() > 1.0 {
                    let locks_at = self.will_lock_at(&self.active).coord.1;
                    let mut new_position = self.active;
                    new_position.coord.1 -= self.gravity_state.trunc() as usize;
                    new_position.coord.1 = std::cmp::max(new_position.coord.1, locks_at);
                    self.gravity_state = self.gravity_state.fract();
                    out.extend(self.reposition(new_position))
                }

                out.into_iter()
                    .map(|action| action.attach_frame((subframe + 9) / 10))
                    .collect_vec()
            })
            .collect_vec()
    }

    /// Tests for whether the active piece is about to lock -- that is, one of its cells is just
    /// above a filled cell. If this is the case, the tetromino will not be allowed to drop any
    /// farther.
    pub fn active_will_lock(&self) -> bool {
        self.will_lock(self.active)
    }

    /// Tests if the given mino intersects a filled cell on the board
    pub fn intersects(&self, mino: &Mino) -> bool {
        mino.position()
            .0
            .iter()
            .any(|&(x, y)| self.cell(x, y).map(|c| !c.is_empty()).unwrap_or(true))
    }

    /// Tests for whether the given mino is in a locking position -- that is, one of its cells is
    /// just above a filled cell. If this is the case, the tetromino will not be allowed to drop any
    /// farther, and, if not hard dropped, the locking countdown will begin.
    fn will_lock(&self, mino: Mino) -> bool {
        mino.position().0.iter().any(|&(x, y)| {
            y.checked_sub(1)
                .and_then(|y| self.cell(x, y))
                .map(|c| !c.is_empty())
                .unwrap_or(true)
        })
    }

    /// Given the current mino, returns a copy of the mino for which calling [Self::will_lock] on it
    /// returns true (that is, gives the mino back at a position at which it would lock). Panics
    /// when the mino is not within the bounds of the board (sometimes. I'm not motivated enough to
    /// figure out the exact bounds where it will panic, so this is an overgeneralization).
    fn will_lock_at(&self, mino: &Mino) -> Mino {
        (0..=mino.coord.1)
            .rev()
            .map(|y| {
                (*mino).tap_mut(|a| {
                    let (x, _) = a.coord;
                    a.coord = (x, y);
                })
            })
            .find(|mino| self.will_lock(*mino))
            .unwrap() // valid if the mino's current position is guaranteed valid
    }

    fn reposition(&mut self, to: Mino) -> Vec<ActionKind> {
        let mut out = vec![ActionKind::Reposition { piece: to }];

        if self.active_will_lock() {
            self.lock_count -= 1;
        }
        self.active = to;
        if self.lock_count == 0 {
            out.extend(self.drop_active());
        }

        out
    }

    /// Drops the active tetromino into the lowest possible position within the columns it takes up.
    pub fn drop_active(&mut self) -> Vec<ActionKind> {
        self.lock_count = 16;
        let dropping = self.cycle_piece();
        let kind: Cell = dropping.variant.into();

        let dropped = self.will_lock_at(&dropping).position();

        // populate the cells which have been dropped into
        dropped.iter().for_each(|position| {
            *self.cell_mut(position.0, position.1).unwrap() = kind.clone();
        });

        // here we check every row, not just the ones dropped into, because tetrio can behave
        // like that (custom boards)
        let dropped_cells = dropped.0.into_iter().map(|(x, y)| ActionKind::Cell {
            position: (x as u8, y as u8),
            kind: kind.clone(),
        });
        dropped_cells.chain(self.clear_lines()).collect_vec()
    }

    fn clear_lines(&mut self) -> impl Iterator<Item = ActionKind> + '_ {
        (0..self.cells.num_rows().0)
            .scan(0, |real_row, _| {
                Some(if self.is_filled(*real_row).unwrap() {
                    // clear the row
                    self.cells.clear_line(*real_row as usize);

                    Some(ActionKind::LineClear {
                        row: *real_row as u8,
                    })
                } else {
                    // this row affects the indices of the rows above since it will not be removed
                    *real_row += 1;
                    None
                })
            })
            .flatten()
    }

    /// Attempts to rotate the active tetromino on the board.
    ///
    /// For now, assumes SRS+
    pub fn rotate_active(&mut self, spin: Spin) -> Vec<ActionKind> {
        let rotated = self.active.rotate(spin);

        let true_rotation = rotated.position();
        let kicks = self
            .active
            .kick(spin)
            .cloned()
            .map(|i| i.into_iter())
            .ok_or(vec![].into_iter()); // where SRS+ assumed

        let accepted_kick = iter::once((0, 0))
            .chain(Either::from(kicks))
            .find(|offset| {
                let testing = true_rotation.clone() + *offset;
                self.test_empty(&testing)
            });

        accepted_kick
            .map(|(x, y)| {
                self.gravity_state = 0.0;

                let new_position = rotated.tap_mut(
                    |Mino {
                         coord: (tet_x, tet_y),
                         ..
                     }| {
                        *tet_x = tet_x.wrapping_add_signed(x as isize);
                        *tet_y = tet_y.wrapping_add_signed(y as isize);
                    },
                );
                self.reposition(new_position)
            })
            .unwrap_or(Vec::new())
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
        positions.iter().any(|&(_, y)| (0..20).contains(&y))
    }

    /// Tests whether or not the positions passed in intersect with the wall or other filled cells
    /// (which may, for example, imply they are available for a tetromino to rotate into) Also
    /// tests within the buffer above the region in which it it legal to place tetrominos
    fn test_empty<const N: usize>(&self, positions: &Positions<N>) -> bool {
        positions.iter().all(|&(x, y)| {
            // check the position is within the lower bounds of the board
            (x >= 0 && y >= 0) &&
            // and that the cell at that position is empty on the board (and within the upper
            // bounds of the board)
                self.cell(x, y)
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

    use itertools::Itertools;

    use super::{storage::BoardStorage, Board, Hold};
    use crate::{board::Cell, rng::PieceQueue};

    use bsr_tools::tetromino::{Direction, Mino, MinoVariant, Spin};

    impl Default for Board {
        fn default() -> Self {
            Self {
                cells: BoardStorage::new_empty(),
                queue: PieceQueue::meaningless(),
                active: MinoVariant::T.into(),
                gravity_state: 0.0,
                lock_count: 16,
                hold: Hold::Empty,
            }
        }
    }

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
                coord: (5, 20),
            },
            ..Default::default()
        };

        board.rotate_active(Spin::CW);
    }

    #[test]
    fn test_t_kicks() {
        let mut tki_board = Board {
            active: Mino {
                variant: MinoVariant::T,
                direction: Direction::Right,
                coord: (1, 2),
            },
            // the flat-top tki made with garbage cells built with tspin on the left
            cells: board_from_string("___________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________###____#__####___#___########_#######"),
            ..Default::default()
        };

        tki_board.rotate_active(Spin::CW);
        assert_eq!(tki_board.active.coord, (2, 1));

        let mut tst_board = Board {
            cells: board_from_string("__________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________#_______________####_#########__########_#####"),
            active: Mino {
                variant: MinoVariant::T,
                direction: Direction::Up,
                coord: (5, 3)
            },
            ..Default::default()
        };

        tst_board.rotate_active(Spin::CW);
        assert_eq!(tst_board.active.coord, (4, 1))
    }

    #[test]
    fn test_drops() {
        {
            // navigate through messy board
            let board_initial = board_from_string("________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________#__########___#########__#____###___#__####______###_____#_##_#_____#_#_#_#_____#_###_#####___######____#####_###_####_#");
            let board_final = board_from_string("________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________#__########___#########__#____###___#__####______###_____#_##_#jjj__#_#_#_#j____#_###_#####___######____#####_###_####_#");

            let mut b = Board {
                cells: board_initial,
                active: Mino {
                    variant: MinoVariant::J,
                    direction: Direction::Down,
                    coord: (4, 7),
                },
                ..Default::default()
            };

            b.drop_active();

            assert_eq!(b.cells, board_final, "messy board");

            // drop that clears lines
            let board_initial = board_from_string("______________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________####_#####__________####_#####____________________");
            let board_final = board_from_string("______________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________tt________________________");

            let mut b = Board {
                cells: board_initial,
                active: Mino {
                    variant: MinoVariant::T,
                    direction: Direction::Right,
                    coord: (4, 3),
                },
                ..Default::default()
            };

            println!("{:?}", b.drop_active());
            assert_eq!(b.cells, board_final, "unnatural t skim")
        }
    }
}
