use std::{collections::HashMap, ops::Add};

use once_cell::sync::Lazy;
use tap::Tap;

use crate::board::{Rotation, MinoVariant, Mino};

#[derive(Debug, Clone)]
pub struct Positions<const N: usize>([(isize, isize); N]);

impl<const N: usize> Add<(i8, i8)> for Positions<N> {
    type Output = Positions<N>;

    fn add(self, (rhs_x, rhs_y): (i8, i8)) -> Self::Output {
        self.tap_mut(|arr| {
            arr.0.iter_mut().for_each(|(x, y)| {
                *x += rhs_x as isize;
                *y += rhs_y as isize;
            })
        })
    }
}

impl<const N: usize> Add<(usize, usize)> for Positions<N> {
    type Output = Positions<N>;

    fn add(self, (rhs_x, rhs_y): (usize, usize)) -> Self::Output {
        self.tap_mut(|arr| {
            arr.0.iter_mut().for_each(|(x, y)| {
                *x += rhs_x as isize;
                *y += rhs_y as isize;
            })
        })
    }
}

impl<const N: usize> Positions<N> {
    pub fn iter(&self) -> impl Iterator<Item = &(isize, isize)> {
        self.0.iter()
    }
}

impl Positions<4> {
    pub fn tetromino(mino: Mino) -> Self {
        let mut cells = [(0, 0); 4];
        cells
            .iter_mut()
            .zip(
                ROTATION_TABLE
                    .get(&(mino.variant, mino.direction))
                    .unwrap(),
            )
            .for_each(|((fin_x, fin_y), (init_x, init_y))| {
                *fin_x = *init_x as isize;
                *fin_y = *init_y as isize;
            });

        Self(cells) + mino.position
    }
}

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

type TetrominoState = (MinoVariant, Rotation);
type KickTable = HashMap<Rotation, Vec<(i8, i8)>>;

fn center_of_mass_rotation(
    piece: MinoVariant,
    up_position: [(i8, i8); 4],
) -> [((MinoVariant, Rotation), [(i8, i8); 4]); 4] {
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
    piece: MinoVariant,
    position: [(i8, i8); 4],
) -> [(TetrominoState, [(i8, i8); 4]); 4] {
    [
        rotation_table!(piece:0 => {position.clone()}),
        rotation_table!(piece:1 => {position.clone()}),
        rotation_table!(piece:2 => {position.clone()}),
        rotation_table!(piece:3 => {position.clone()}),
    ]
}

pub static ROTATION_TABLE: Lazy<HashMap<TetrominoState, [(i8, i8); 4]>> = Lazy::new(|| {
    use MinoVariant::*;

    [
        center_of_mass_rotation(T, [(-1, 0), (0, 0), (1, 0), (0, 1)]),
        center_of_mass_rotation(L, [(-1, -1), (-1, 0), (0, 0), (1, 0)]),
        center_of_mass_rotation(J, [(1, 1), (-1, 0), (0, 0), (1, 0)]),
        static_rotation(O, [(0, 0), (0, 1), (1, 0), (1, 1)]),
        center_of_mass_rotation(S, [(0, 0), (-1, 0), (0, 1), (1, 1)]),
        center_of_mass_rotation(Z, [(0, 0), (1, 0), (0, -1), (-1, -1)]),
        [
            ((I, Rotation::Up), [(-2, 0), (-1, 0), (0, 0), (1, 0)]),
            ((I, Rotation::Left), [(0, 1), (0, 0), (0, -1), (0, -2)]),
            (
                (I, Rotation::Down),
                [(-2, -1), (-1, -1), (0, -1), (1, -1)],
            ),
            (
                (I, Rotation::Right),
                [(-1, 1), (-1, 0), (-1, -1), (-1, -2)],
            ),
        ],
    ]
    .into_iter()
    .flatten()
    .collect()
});

pub static SRS_PLUS_KICK_TABLE: Lazy<KickTable> = Lazy::new(|| {
    use MinoVariant::*;

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
