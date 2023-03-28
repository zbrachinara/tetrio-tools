use std::{collections::VecDeque, iter};

use gridly::prelude::{Column, Grid, GridBounds, GridMut, Row};
use if_chain::if_chain;
use itertools::{Either, Itertools};
use tap::Tap;
use ttrm::{event::InteractionData, GameType};

use crate::{
    reconstruct::{ShiftDirection, State},
    rng::PieceQueue,
};
use viewtris::{
    action::{Action, ActionKind},
    positions::Positions,
    tetromino::{Cell, Mino, MinoVariant, Spin},
};

use self::{settings::Settings, storage::BoardStorage};

pub mod settings;
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

#[derive(Clone, Copy, Debug)]
pub struct Garbage {
    amt: u16,
    column: u16,
    received_frame: u32,
}

/// Holds the state of the tetrio board, which can be updated through the issuing of commands. The
/// board does not keep track of its own state over time. Instead, it transforms the commands it is
/// issued into [Action]s (or [ActionKind]s, which can then be assigned frames), which are returned
/// after each command. This is used to externally build up a sequence of actions while having a
/// copy of the state with which to determine which actions are possible (such as the validity of
/// rotation) and which actions would happen as a consequence of previous actions (such as line
/// clears).
pub struct Board {
    /// The double array representing the "board" part of the game, consisting of all the cells
    /// which result from pieces having been locked down (does not include the *currently* active
    /// piece).
    pub matrix: BoardStorage<Cell>,
    pub queue: PieceQueue,
    pub active: Mino,
    /// A value signifying how much time has passed since the active piece has most recently fallen
    /// (by any amount). If this piece surpasses a certain threshold, the excess is used to
    /// calculate how far this piece should fall, and whether or not it should lock in place.
    pub gravity_state: f64,
    settings: Settings,
    /// How many times the piece is able to avoid locking until it is forced to lock immediately.
    lock_count: i8,
    /// How many subframes the active piece has remained in a locking position. Used to caclulate
    /// whether a piece should lock due to the expiration of lock delay.
    lock_timer: u32,
    /// The most recent subframe the active piece was dropped on (specifically, the subframe time of
    /// the latest call to `Board::drop_active`)
    // TODO "latest call" is not necessarily true, see usages of `last_drop_needs_update`
    last_drop: Option<u32>,
    last_drop_needs_update: bool, // TODO way too hacky, redo api to fix this
    hold: Hold,
    acknowledged_garbage: VecDeque<Garbage>, // TODO maybe more performant to combine the vecdeques
    queued_garbage: VecDeque<Garbage>,
}

impl Board {
    /// Creates a new board from a PRNG seed and a board that may be filled with some cells.
    ///
    /// The format of the matrix is the same as the format found in ttr and ttrm files -- that is,
    /// as a two-dimensional matrix.
    pub fn new(
        piece_seed: u64,
        game_type: GameType,
        settings: Settings,
        game: &[Vec<Option<&str>>],
    ) -> (Self, Vec<Action>) {
        let mut queue = PieceQueue::from_game(game_type, piece_seed);
        let cells = BoardStorage::new_from_rows_unchecked(
            game.iter()
                .map(|row| row.iter().map(|elem| Cell::from(*elem)).collect_vec())
                .rev()
                .collect_vec(),
        );

        let active = queue.pop().into();

        (
            Self {
                matrix: cells,
                queue,
                active,
                gravity_state: 0.0,
                settings,
                lock_count: 16,
                lock_timer: 0,
                hold: Hold::Empty,
                last_drop: None,
                last_drop_needs_update: false,
                acknowledged_garbage: VecDeque::new(),
                queued_garbage: VecDeque::new(),
            },
            vec![ActionKind::Reposition { piece: active }.attach_frame(0)],
        )
    }

    /// Expends and returns the currently active piece, replacing it with the next piece in the
    /// queue. Meant for internal use, such as during holding/after hard dropping.
    fn cycle_piece(&mut self) -> Mino {
        std::mem::replace(&mut self.active, Mino::from(self.queue.pop()))
    }

    /// Shifts the active tetromino by the given amount of cells.
    pub fn shift(&mut self, cells: i8) -> Vec<ActionKind> {
        let shift_to = self.active.coord.0 + cells as i16;

        // ranges are inclusive since the tetromino can at least occupy its current position
        let shift_through = if self.active.coord.0 < shift_to {
            itertools::Either::Left(self.active.coord.0..=shift_to)
        } else {
            itertools::Either::Right((shift_to..=self.active.coord.0).rev())
        }
        .map(|x| self.active.tap_mut(|piece| piece.coord.0 = x))
        .tuple_windows()
        .enumerate();

        let mut new_position = None;

        for (ix, (m1, m2)) in shift_through {
            if self.intersects(&m2) {
                new_position = Some(m1);
                break;
            }

            // This if statement is positioned here to avoid being called at the beginning of the
            // shift (ix != 0) and at the end of the shift (since the loop will break before that
            // happens)
            if ix != 0 && self.will_lock(m1) {
                self.lock_count -= 1;
            }
        }

        let new_position =
            new_position.unwrap_or_else(|| self.active.tap_mut(|piece| piece.coord.0 = shift_to));

        self.reposition(new_position)
    }

    pub fn acknowledge_garbage(&mut self, garbage: &InteractionData, frame: u32) {
        match *garbage {
            InteractionData::InteractionDo { .. } => (), // ignore for now
            InteractionData::InteractionConfirm {
                data: ttrm::event::Garbage { amt, column, .. },
            } => self.acknowledged_garbage.push_back(Garbage {
                amt,
                column,
                received_frame: frame,
            }),
        }
    }

    /// Holds a piece, returning the proper actions depending on whether holding is possible or not.
    pub fn hold(&mut self) -> Vec<ActionKind> {
        match self.hold {
            Hold::Empty => {
                self.hold = Hold::NotActive(self.cycle_piece().variant);
                vec![
                    ActionKind::Hold,
                    ActionKind::Reposition { piece: self.active },
                ]
            }
            Hold::Active(held) => {
                self.hold =
                    Hold::NotActive(std::mem::replace(&mut self.active, held.into()).variant);
                vec![
                    ActionKind::Hold,
                    ActionKind::Reposition { piece: self.active },
                ]
            }
            Hold::NotActive(_) => vec![],
        }
    }

    /// Calculates whether or not DAS has been held long enough to shift the current piece (or, if a
    /// piece has just been dropped, if das cut has been passed). If the piece is ready to shift,
    /// returns the strength of the shift.
    fn auto_shift_charge(
        &self,
        current_subframe: u32,
        settings: &Settings,
        key_state: &State,
    ) -> Option<i8> {
        let (arr, shift_size) = if settings.arr == 0 {
            (1, self.matrix.num_columns().0 as i8)
        } else {
            (settings.arr, 1)
        };

        let das_inertia = if_chain!(
            if let Some(last_drop) = self.last_drop;
            if last_drop >= key_state.shift_began + settings.das;
            then {
                last_drop + settings.dcd
            } else {
                key_state.shift_began + settings.das
            }
        );

        current_subframe
            .checked_sub(das_inertia)
            .map(|time_after_das| time_after_das % arr == 0)
            .unwrap_or(false)
            .then_some(shift_size)
    }

    /// Handles effects that happen between keypresses, such as auto-shift, gravity, and soft drop.
    /// These effects must be handled together because each of them changes the position of the
    /// mino, which in turn changes which shifts and drops are possible.
    // TODO The order in which effects are applied is not verified
    pub fn passive_effects(&mut self, current_subframe: u32, key_state: &State) -> Vec<Action> {
        (key_state.last_subframe..current_subframe)
            .flat_map(|subframe| {
                let settings = &self.settings;
                let frame = subframe / 10;

                // TODO handle gravity acceleration
                self.gravity_state += if key_state.soft_dropping {
                    if settings.sdf > 40 {
                        self.matrix.num_rows().0 as f64 * 10.
                    } else {
                        let gravity_base = f64::max(settings.gravity, 0.05);
                        settings.sdf as f64 * gravity_base
                    }
                } else {
                    settings.gravity
                } / 10.;

                let mut out = Vec::new();

                if self.last_drop_needs_update {
                    self.last_drop_needs_update = false;
                    self.last_drop = Some(subframe)
                }

                // activate garbage if ready
                // TODO garbage cancellation
                while let Some(garbage) = self.acknowledged_garbage.get(0) {
                    if garbage.received_frame + settings.garbage_speed - 1 <= frame {
                        self.queued_garbage
                            .push_back(self.acknowledged_garbage.pop_front().unwrap());
                    } else {
                        break;
                    }
                }

                if let Some(shift_size) = self.auto_shift_charge(subframe, settings, key_state) {
                    out.extend(match key_state.shifting {
                        ShiftDirection::None => Vec::new(),
                        ShiftDirection::Left => self.shift(-shift_size),
                        ShiftDirection::Right => self.shift(shift_size),
                    });
                }

                if self.gravity_state >= 1.0 {
                    let locks_at = self.will_lock_at(&self.active).coord.1;
                    let mut new_position = self.active;
                    new_position.coord.1 -= self.gravity_state.trunc() as i16;
                    new_position.coord.1 = std::cmp::max(new_position.coord.1, locks_at);
                    self.gravity_state = self.gravity_state.fract();
                    out.extend(self.reposition(new_position))
                }

                if self.active_will_lock() {
                    self.lock_timer += 1;
                    if self.lock_timer >= 300 {
                        out.extend(self.drop_active());
                    }
                }

                out.into_iter()
                    .map(move |action| action.attach_frame(subframe / 10))
            })
            .collect_vec()
    }

    /// Tests for whether the active piece is about to lock -- that is, one of its cells is just
    /// above a filled cell. If this is the case, the tetromino will not be allowed to drop any
    /// farther.
    fn active_will_lock(&self) -> bool {
        self.will_lock(self.active)
    }

    /// Tests if the given mino intersects a filled cell on the board
    pub fn intersects(&self, mino: &Mino) -> bool {
        mino.position()
            .0
            .iter()
            .any(|&(x, y)| self.cell(x, y).map(|c| !c.is_empty()).unwrap_or(true))
    }

    /// Tests for whether the given mino is is just above a filled cell. If this is the case, the
    /// tetromino will not be allowed to drop any farther, and, if not hard dropped, the locking
    /// countdown will begin.
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

    /// Repositions the piece to the given position, locking it if necessary.
    fn reposition(&mut self, to: Mino) -> Vec<ActionKind> {
        let mut out = vec![];

        if to != self.active {
            self.lock_timer = 0;
            if self.active_will_lock() {
                self.lock_count -= 1;
            }
            out.push(ActionKind::Reposition { piece: to });
            self.active = to;
        }

        if self.lock_count <= 0 {
            out.extend(self.drop_active());
        }
        out
    }

    /// Drops the active tetromino in the usual way.
    pub fn drop_active(&mut self) -> Vec<ActionKind> {
        self.lock_count = 16;
        self.lock_timer = 0;
        let dropping = self.cycle_piece();
        let kind: Cell = dropping.variant.into();

        let dropped = self.will_lock_at(&dropping).position();

        // populate the cells which have been dropped into
        dropped.iter().for_each(|&(x, y)| {
            *self.cell_mut(x, y).unwrap() = kind;
        });

        let dropped_cells = dropped.0.into_iter().map(|(x, y)| ActionKind::Cell {
            position: (x as u8, y as u8),
            kind,
        });

        let active = self.active;
        self.hold.activate();
        self.last_drop_needs_update = true;

        let new_lines = {
            let cleared_lines = self.clear_lines();
            if cleared_lines.is_empty() {
                self.apply_queued_garbage()
            } else {
                cleared_lines
            }
        };

        dropped_cells
            .chain(new_lines)
            .chain(std::iter::once(ActionKind::Reposition { piece: active }))
            .collect_vec()
    }

    /// Add all garbage to the matrix which has been acknowledged and passed the necessary delay.
    /// Also cuts off any blocks which exceeds the garbage cap, and may cut block in half if it is
    /// necessary.
    fn apply_queued_garbage(&mut self) -> Vec<ActionKind> {
        let mut out = vec![];
        let mut counter = 0;

        while let Some(garbage) = self.queued_garbage.get(0) {
            // TODO get garbage cap from settings
            if counter + garbage.amt > self.settings.garbage_cap {
                let excess = counter + garbage.amt - 8;
                let applied = garbage.amt - excess;

                out.push(self.apply_garbage((*garbage).tap_mut(|gb| gb.amt = applied)));
                self.queued_garbage[0].amt = excess;
                break;
            } else {
                let gb = self.queued_garbage.pop_front().unwrap();
                counter += gb.amt;
                out.push(self.apply_garbage(gb));
                if counter == 8 {
                    break;
                }
            }
        }

        out
    }

    /// Apply one block of garbage
    fn apply_garbage(&mut self, garbage: Garbage) -> ActionKind {
        self.matrix.apply_garbage(garbage.column, garbage.amt);
        ActionKind::Garbage {
            column: garbage.column,
            height: garbage.amt,
        }
    }

    /// Checks each row in the board, and removes any row which is full. Every row is checked, not
    /// just updated ones, because tetrio can behave like that (for example, on custom boards that
    /// init with some filled rows)
    fn clear_lines(&mut self) -> Vec<ActionKind> {
        (0..self.matrix.num_rows().0)
            .scan(0, |real_row, _| {
                Some(if self.is_filled(*real_row).unwrap() {
                    // clear the row
                    self.matrix.clear_line(*real_row as usize);

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
            .collect_vec()
    }

    /// Attempts to rotate the active tetromino on the board.
    ///
    /// For now, assumes SRS+
    pub fn rotate_active(&mut self, spin: Spin) -> Vec<ActionKind> {
        let rotated = self.active.rotate(spin); // where SRS+ assumed

        let true_rotation = rotated.position();
        let kicks = self
            .active
            .kick(spin)
            .cloned()
            .map(|i| i.into_iter())
            .ok_or(vec![].into_iter());

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
                        *tet_x += x as i16;
                        *tet_y += y as i16;
                    },
                );
                self.reposition(new_position)
            })
            .unwrap_or(Vec::new())
    }

    /// Tests if a row is filled, and therefore should be cleared. Returns `None` if the given row
    /// is invalid
    fn is_filled(&self, row: isize) -> Option<bool> {
        self.matrix
            .row(row)
            .ok()
            .map(|row| !row.iter().any(|cell| cell.is_empty()))
    }

    /// Tests whether or not the positions passed in intersect with the wall or other filled cells
    /// (which may, for example, imply they are available for a tetromino to rotate into) Also tests
    /// within the buffer above the region in which it it legal to place tetrominos
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
        self.matrix.get((Column(x), Row(y))).ok()
    }

    fn cell_mut(&mut self, x: isize, y: isize) -> Option<&mut Cell> {
        self.matrix.get_mut((Column(x), Row(y))).ok()
    }
}

#[cfg(test)]
mod test {

    use std::collections::VecDeque;

    use itertools::Itertools;

    use super::{settings::Settings, storage::BoardStorage, Board, Hold};
    use crate::{board::Cell, rng::PieceQueue};

    use viewtris::tetromino::{Direction, Mino, MinoVariant, Spin};

    impl Default for Board {
        fn default() -> Self {
            Self {
                matrix: BoardStorage::new_empty(),
                queue: PieceQueue::meaningless(),
                active: MinoVariant::T.into(),
                gravity_state: 0.0,
                settings: Settings::default(),
                lock_count: 16,
                lock_timer: 0,
                hold: Hold::Empty,
                last_drop: None,
                last_drop_needs_update: false,
                acknowledged_garbage: VecDeque::new(),
                queued_garbage: VecDeque::new(),
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
            matrix: board_from_string("___________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________###____#__####___#___########_#######"),
            ..Default::default()
        };

        tki_board.rotate_active(Spin::CW);
        assert_eq!(tki_board.active.coord, (2, 1));

        let mut tst_board = Board {
            matrix: board_from_string("__________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________#_______________####_#########__########_#####"),
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
                matrix: board_initial,
                active: Mino {
                    variant: MinoVariant::J,
                    direction: Direction::Down,
                    coord: (4, 7),
                },
                ..Default::default()
            };

            b.drop_active();

            assert_eq!(b.matrix, board_final, "messy board");

            // drop that clears lines
            let board_initial = board_from_string("______________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________####_#####__________####_#####____________________");
            let board_final = board_from_string("______________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________tt________________________");

            let mut b = Board {
                matrix: board_initial,
                active: Mino {
                    variant: MinoVariant::T,
                    direction: Direction::Right,
                    coord: (4, 3),
                },
                ..Default::default()
            };

            println!("{:?}", b.drop_active());
            assert_eq!(b.matrix, board_final, "unnatural t skim")
        }
    }
}
