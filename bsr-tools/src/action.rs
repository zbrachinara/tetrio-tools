use crate::tetromino::{Cell, Mino};

/// An action is something which can happen to a player's board, though it is not recorded whether
/// or not this is a consequence of the player's actions themselves. They may happen in the same
/// frame, as in the case of a hard drop or piece locking into place and spawning multiple cells,
/// but will not be executed simultaneously. Instead, they will be executed in the order of the
/// frames first, and then in the order that they are given.
#[derive(Debug, Clone)]
pub enum ActionKind {
    /// Create a new garbage line with a hole at the specified column with the given height.
    Garbage { column: u8, height: u8 },
    /// Updates the state of the active mino. The possibility of this update is not checked, so
    /// reposition at your own risk
    Reposition { piece: Mino },
    /// Removes a line at the given row
    LineClear { row: u8 },
    /// Changes a cell at the given position. This is not limited to spawning a cell, and can also
    /// mark one's removal
    Cell { position: (u8, u8), kind: Cell },
    /// Activates the hold function, which usually means swapping the active piece with a piece in
    /// an independently managed queue (usually one piece long).
    Hold,
}

impl ActionKind {
    pub fn attach_frame(self, frame: u64) -> Action {
        Action { kind: self, frame }
    }
}

#[derive(Debug)]
pub struct Action {
    pub kind: ActionKind,
    pub frame: u64,
}
