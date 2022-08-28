use std::{collections::HashMap};

use once_cell::sync::{Lazy};

use crate::board::{Board, Direction, TetrominoVariant, Rotation, RotationState};

static SRS_KICK_TABLE: Lazy<KickTable> = Lazy::new(|| {
    use TetrominoVariant::*;
    use RotationState::*;

    [J, L, T, S, Z].into_iter().map(|variant| {
        [(Rotation {
            piece: variant,
            from: Up, 
            to: Right,
        }, vec![(-1, 0)]),
        (Rotation {
            piece: variant,
            from: Right,
            to: Up,
        }, vec![(1, 0)])]
    }).flatten().collect()
});

type KickTable = HashMap<Rotation, Vec<(i8, i8)>>;

impl Board {
    /// Attempts to rotate the active tetromino on the board. Returns true if successful,
    /// false otherwise.
    ///
    /// For now, assumes SRS+
    fn rotate_active(&mut self, direction: Direction) -> bool {
        todo!()
    }
}
