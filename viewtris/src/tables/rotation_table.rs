use std::collections::HashMap;

use once_cell::sync::Lazy;
use tap::Tap;

use crate::tetromino::{Direction, MinoVariant};

type TetrominoState = (MinoVariant, Direction);

macro_rules! rotation_table {
    ($piece:ident:$rot:literal => $list:tt) => {{
        (($piece, $rot.into()), $list)
    }};
}

fn center_of_mass_rotation(
    piece: MinoVariant,
    up_position: [(i8, i8); 4],
) -> [(TetrominoState, [(i8, i8); 4]); 4] {
    [
        rotation_table!(piece:0 => {up_position}), // normal
        rotation_table!(piece:1 => {
            up_position.tap_mut(|positions| { positions
                .iter_mut().for_each(|coords| *coords = (coords.1, -coords.0))
            })
        }),
        rotation_table!(piece:2_i8 => {
            up_position.tap_mut(|positions| { positions
                .iter_mut().for_each(|coords| *coords = (-coords.0, -coords.1))
            })
        }),
        rotation_table!(piece:3_i8 => {
            up_position.tap_mut(|positions| { positions
                .iter_mut().for_each(|coords| *coords = (-coords.1, coords.0))
            })
        }),
    ]
}

fn static_rotation(
    piece: MinoVariant,
    position: [(i8, i8); 4],
) -> [(TetrominoState, [(i8, i8); 4]); 4] {
    [
        rotation_table!(piece:0 => {position}),
        rotation_table!(piece:1 => {position}),
        rotation_table!(piece:2 => {position}),
        rotation_table!(piece:3 => {position}),
    ]
}

pub static ROTATION_TABLE: Lazy<HashMap<TetrominoState, [(i8, i8); 4]>> = Lazy::new(|| {
    use MinoVariant::*;

    [
        center_of_mass_rotation(T, [(-1, 0), (0, 0), (1, 0), (0, 1)]),
        center_of_mass_rotation(L, [(1, 1), (-1, 0), (0, 0), (1, 0)]),
        center_of_mass_rotation(J, [(-1, 1), (-1, 0), (0, 0), (1, 0)]),
        static_rotation(O, [(0, 0), (0, 1), (1, 0), (1, 1)]),
        center_of_mass_rotation(S, [(0, 0), (-1, 0), (0, 1), (1, 1)]),
        center_of_mass_rotation(Z, [(0, 0), (1, 0), (0, 1), (-1, 1)]),
        [
            ((I, Direction::Up), [(2, 0), (-1, 0), (0, 0), (1, 0)]),
            ((I, Direction::Left), [(0, 1), (0, 0), (0, -1), (0, -2)]),
            ((I, Direction::Down), [(2, -1), (-1, -1), (0, -1), (1, -1)]),
            ((I, Direction::Right), [(1, 1), (1, 0), (1, -1), (1, -2)]),
        ],
    ]
    .into_iter()
    .flatten()
    .collect()
});
