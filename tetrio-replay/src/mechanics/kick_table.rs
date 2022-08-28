use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::board::{TetrominoVariant, RotationState, Rotation};


macro_rules! kick_table {
    () => {
        
    };
}

type KickTable = HashMap<Rotation, Vec<(i8, i8)>>;

#[rustfmt::skip]
static SRS_KICK_TABLE: Lazy<KickTable> = Lazy::new(|| {
    use TetrominoVariant::*;
    use RotationState::*;

    [J, L, T, S, Z].into_iter().map(|variant| {
        [
            (Rotation {
                piece: variant,
                from: Up, 
                to: Right,
            }, vec![(-1, 0), (-1, 1), (0, -2), (-1, -2)]),
            (Rotation {
                piece: variant,
                from: Right,
                to: Up,
            }, vec![(1, 0), (1, -1), (0, 2), (1, 2)]),
            (Rotation {
                piece: variant,
                from: Right,
                to: Down,
            }, vec![(1, 0), (1, -1), (0, 2), (1, 2)]),
            (Rotation {
                piece: variant,
                from: Down,
                to: Right,
            }, vec![])
        ]
    }).flatten().collect()
});
