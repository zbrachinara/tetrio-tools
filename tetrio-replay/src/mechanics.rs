use crate::board::{Board, Direction};

mod kick_table;

impl Board {
    /// Attempts to rotate the active tetromino on the board. Returns true if successful,
    /// false otherwise.
    ///
    /// For now, assumes SRS+
    fn rotate_active(&mut self, direction: Direction) -> bool {
        todo!()
    }
}
