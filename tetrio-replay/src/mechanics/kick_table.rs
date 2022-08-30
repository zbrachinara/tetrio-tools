use std::collections::HashMap;

use once_cell::sync::Lazy;
use tap::Tap;

use crate::board::{Rotation, RotationState, TetrominoVariant};

macro_rules! kick_table {
    ($piece:ident:$from:literal>>$to:literal => $list:tt) => {
        {
            (
                Rotation { piece: $piece, from: $from.into(), to: $to.try_into().unwrap()},
                vec!$list
            )
        }
    };
}

macro_rules! rotation_table {
    ($piece:ident:$rot:literal => $list:tt) => {{
        (($piece, $rot.into()), $list)
    }};
}

type TetrominoState = (TetrominoVariant, RotationState);
type KickTable = HashMap<Rotation, Vec<(i8, i8)>>;

fn center_of_mass_rotation(
    piece: TetrominoVariant,
    up_position: [(i8, i8); 4],
) -> [((TetrominoVariant, RotationState), [(i8, i8); 4]); 4] {
    [
        rotation_table!(piece:0 => {up_position.clone()}), // normal
        rotation_table!(piece:1 => {
            up_position.clone().tap_mut(|positions| { positions
                .iter_mut().for_each(|coords| *coords = (coords.1, -coords.0))
            })
        }),
        rotation_table!(piece:2_i8 => {
            up_position.clone().tap_mut(|positions| { positions
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
    piece: TetrominoVariant,
    position: [(i8, i8); 4],
) -> [(TetrominoState, [(i8, i8); 4]); 4] {
    [
        rotation_table!(piece:0 => {position.clone()}),
        rotation_table!(piece:1 => {position.clone()}),
        rotation_table!(piece:2 => {position.clone()}),
        rotation_table!(piece:3 => {position.clone()}),
    ]
}

static ROTATION_TABLE: Lazy<HashMap<TetrominoState, [(i8, i8); 4]>> = Lazy::new(|| {
    use TetrominoVariant::*;

    [
        center_of_mass_rotation(T, [(-1, 0), (0, 0), (1, 0), (0, 1)]),
        center_of_mass_rotation(L, [(-1, -1), (-1, 0), (0, 0), (1, 0)]),
        center_of_mass_rotation(J, [(1, 1), (-1, 0), (0, 0), (1, 0)]),
        static_rotation(O, [(0, 0), (0, 1), (1, 0), (1, 1)]),
        center_of_mass_rotation(S, [(0, 0), (-1, 0), (0, 1), (1, 1)]),
        center_of_mass_rotation(Z, [(0, 0), (1, 0), (0, -1), (-1, -1)]),
        [
            ((I, RotationState::Up), [(-2, 0), (-1, 0), (0, 0), (1, 0)]),
            ((I, RotationState::Left), [(0, 1), (0, 0), (0, -1), (0, -2)]),
            ((I, RotationState::Down), [(-2, -1), (-1, -1), (0, -1), (1, -1)]),
            ((I, RotationState::Right), [(-1, 1), (-1, 0), (-1, -1), (-1, -2)]),
        ]
    ]
    .into_iter()
    .flatten()
    .collect()
});

static SRS_PLUS_KICK_TABLE: Lazy<KickTable> = Lazy::new(|| {
    use TetrominoVariant::*;

    [J, L, T, S, Z]
        .into_iter()
        .map(|variant| {
            // srs standard kicks
            [
                kick_table!(variant:0>>1 => [(-1, 0), (-1, 1), (0, -2), (-1, -2)]),
                kick_table!(variant:1>>0 => [(1, 0), (1, -1), (0, 2), (1, 2)]),
                kick_table!(variant:1>>2 => [(1, 0), (1, -1), (0, 2), (1, 2)]),
                kick_table!(variant:2>>1 => [(-1, 0), (-1, 1), (0, 2), (-1, -2)]),
                kick_table!(variant:2>>3 => [(1, 0), (1, 1), (0, -2), (1, -2)]),
                kick_table!(variant:3>>2 => [(-1, 0), (-1, -1), (0, 2), (-1, 2)]),
                kick_table!(variant:3>>0 => [(-1, 0), (-1, -1), (0, 2), (-1, 2)]),
                kick_table!(variant:0>>3 => [(1, 0), (1, 1), (0, -2), (1, -2)]),
            ]
        })
        .flatten()
        // the following rotations are specific to SRS+
        .chain([
            // I CW/CCW rotation kick table
            kick_table!(I:0>>1 => [(1, 0), (-2, 0), (1, -2), (-2, 1)]),
            kick_table!(I:1>>0 => [(-1, 0), (2, 0), (-1, 2), (2, -1)]),
            kick_table!(I:1>>2 => [(-1, 0), (2, 0), (-1, -2), (2, 1)]),
            kick_table!(I:2>>1 => [(-2, 0), (1, 0), (-2, -1), (1, 2)]),
            kick_table!(I:2>>3 => [(2, 0), (-1, 0), (2, -1), (-1, 2)]),
            kick_table!(I:3>>2 => [(1, 0), (-2, 0), (1, -2), (-2, 1)]),
            kick_table!(I:3>>0 => [(1, 0), (-2, 0), (1, 2), (-2, -1)]),
            kick_table!(I:0>>3 => [(-1, 0), (2, 0), (-1, -2), (2, 1)]),
            // I 180 kick table
            kick_table!(I:0>>2 => [(0, -1)]),
            kick_table!(I:1>>3 => [(1, 0)]),
            kick_table!(I:2>>0 => [(0, 1)]),
            kick_table!(I:3>>1 => [(-1, 0)]),
        ])
        .chain([
            // T 180 rotation table
            kick_table!(T:0>>2 => [(0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0)]),
            kick_table!(T:2>>0 => [(0, -1), (-1, -1), (1, -1), (-1, 0), (1, 0)]),
            kick_table!(T:1>>3 => [(1, 0), (1, 2), (1, 1), (0, 2), (0, 1)]),
            kick_table!(T:3>>1 => [(-1, 0), (-1, 2), (-1, 1), (0, 2), (0, 1)]),
        ])
        .collect()
});
