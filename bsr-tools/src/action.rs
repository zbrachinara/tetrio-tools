use crate::tetromino::{Cell, Mino};

/// An action is something which can happen to a player's board, though it is not recorded whether
/// or not this is a consequence of the player's actions themselves. They may happen in the same
/// frame, as in the case of a hard drop or piece locking into place and spawning multiple cells,
/// but will not be executed simultaneously. Instead, they will be executed in the order of the
/// frames first, and then in the order that they are given.
#[derive(Debug)]
pub enum Action {
    /// Create a new garbage line with a hole at the specified column with the given height. 
    Garbage { column: u8, height: u8 },
    /// Does a modification to the active mino. This is not limited to a rotation or translation,
    /// and can also involve changing the type of mino.
    Reposition { piece: Mino },
    /// Removes a line at the given column
    LineClear { row: u8 },
    /// Changes a cell at the given position. This is not limited to spawning a cell, and can also
    /// mark one's removal
    Cell { position: (u8, u8), kind: Cell },
    /// Activates the hold function, which usually means swapping the active piece with a piece in
    /// an independently managed queue (usually one piece long).
    Hold,
}
