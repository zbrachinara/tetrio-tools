use std::{ops::Add, str::FromStr};

use strum::EnumString;
use tap::Tap;

use crate::kick_table::{Positions, ROTATION_TABLE};

#[derive(Clone, Debug)]
pub enum Cell {
    Tetromino(MinoVariant),
    Garbage,
    None,
}

impl<'a> From<Option<&'a str>> for Cell {
    fn from(name: Option<&'a str>) -> Self {
        name.map(|str| {
            if str == "gb" {
                Self::Garbage
            } else {
                Self::Tetromino(<_>::from_str(str).unwrap())
            }
        })
        .unwrap_or(Self::None)
    }
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::None)
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Rotation {
    pub piece: MinoVariant,
    pub from: Direction,
    pub to: Direction,
}

#[derive(Copy, Clone)]
#[repr(i8)]
pub enum Spin {
    CW = 1,
    CCW = 3,
    /// Represents a 180 degree rotation
    Flip = 2,
}

#[repr(i8)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl From<i8> for Direction {
    fn from(n: i8) -> Self {
        unsafe { std::mem::transmute(n % 4) }
    }
}

impl Add<Spin> for Direction {
    type Output = Direction;

    fn add(self, rhs: Spin) -> Self::Output {
        (self as i8 + rhs as i8).into()
    }
}

#[derive(Clone)]
pub struct Mino {
    pub variant: MinoVariant,
    pub direction: Direction,
    pub center: (usize, usize),
}

impl From<MinoVariant> for Mino {
    fn from(variant: MinoVariant) -> Self {
        Self {
            variant,
            direction: Direction::Up,
            center: (5, 22), //TODO: Find out if the piece actually spawns here initially
        }
    }
}

impl Mino {
    pub fn position(&self) -> Positions<4> {
        Positions(
            ROTATION_TABLE
                .get(&(self.variant, self.direction))
                .unwrap()
                .map(|(x, y)| {
                    (
                        self.center.0 as isize + x as isize,
                        self.center.1 as isize + y as isize,
                    )
                }),
        )
    }

    pub fn rotation(&self, at: Spin) -> Rotation {
        Rotation {
            piece: self.variant,
            from: self.direction,
            to: self.direction + at,
        }
    }

    pub fn rotate(&self, at: Spin) -> Self {
        self.clone().tap_mut(|tet| {
            tet.direction = tet.direction + at;
        })
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum MinoVariant {
    L, J, T, Z, S, O, I
}
