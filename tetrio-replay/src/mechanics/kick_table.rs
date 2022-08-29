use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::board::{Rotation, TetrominoVariant};

macro_rules! kick_table {
    ($piece:ident:$from:literal>>$to:literal => $list:tt) => {
        {
            (
                Rotation { piece: $piece, from: $from.try_into().unwrap(), to: $to.try_into().unwrap()},
                vec!$list
            )
        }
    };
}

type KickTable = HashMap<Rotation, Vec<(i8, i8)>>;

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
        .collect()
});
