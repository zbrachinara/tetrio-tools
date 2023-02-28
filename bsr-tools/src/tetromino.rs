use std::{fmt::Display, ops::Add, str::FromStr};

use strum::EnumString;
use tap::Tap;

use crate::kick_table::{self, Positions, ROTATION_TABLE};

impl From<MinoVariant> for Cell {
    fn from(value: MinoVariant) -> Self {
        Self::Tetromino(value)
    }
}

/// The possible states that a cell can take up. A Tetromino cell refers to a filled cell with the
/// color associated with that tetromino
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Tetromino(MinoVariant),
    Garbage,
    Empty,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Tetromino(MinoVariant::I) => f.write_str("II"),
            Cell::Tetromino(MinoVariant::J) => f.write_str("JJ"),
            Cell::Tetromino(MinoVariant::L) => f.write_str("LL"),
            Cell::Tetromino(MinoVariant::S) => f.write_str("SS"),
            Cell::Tetromino(MinoVariant::Z) => f.write_str("ZZ"),
            Cell::Tetromino(MinoVariant::T) => f.write_str("TT"),
            Cell::Tetromino(MinoVariant::O) => f.write_str("OO"),
            Cell::Garbage => f.write_str("GB"),
            Cell::Empty => f.write_str("NC"),
        }
    }
}

// TODO: Tetrio-specific implementation -- separate
impl<'a> From<Option<&'a str>> for Cell {
    fn from(name: Option<&'a str>) -> Self {
        name.map(|str| {
            if str == "gb" {
                Self::Garbage
            } else {
                Self::Tetromino(<_>::from_str(str).unwrap())
            }
        })
        .unwrap_or(Self::Empty)
    }
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
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

/// Represents the orientation of the mino. For example, for a T-mino, the Up direction
/// refers to the orientation where the cell portruding from the center by itself points upward.
/// It can be modified by adding a [Spin] to it, modifying the orientation by the spin described
/// by it.
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

/// A representation of a mino by its states. Does not directly allow you to access its occupied
/// positions (can be done using [Positions]), but does let you easily modify its state.
#[derive(Clone, Copy, Debug)]
pub struct Mino {
    /// The type of mino that is represented (determines its shape)
    pub variant: MinoVariant,
    /// The direction in which the mino is pointing
    pub direction: Direction,
    /// The coordinate is defined by rotation table entries. It is an arbitrary point which can
    /// sometimes be used as the "center of rotation", but its most useful property is that it
    /// is the final degree of freedom that determines the absolute position of the tetromino
    /// relative to the board after its variant and direction.
    pub coord: (usize, usize),
}

impl From<MinoVariant> for Mino {
    fn from(variant: MinoVariant) -> Self {
        Self {
            variant,
            direction: Direction::Up,
            coord: (5, 22), //TODO: Find out if the piece actually spawns here initially
        }
    }
}

impl Mino {
    pub fn position(&self) -> Positions<4> {
        Positions(
            ROTATION_TABLE
                .get(&(self.variant, self.direction))
                .unwrap()
                .map(|(x, y)| (x as isize, y as isize)),
        ) + self.coord
    }

    pub fn kick(&self, at: Spin) -> Option<&Vec<(i8, i8)>> {
        kick_table::SRS_PLUS.get(&Rotation {
            piece: self.variant,
            from: self.direction,
            to: self.direction + at,
        })
    }

    pub fn rotation(&self, at: Spin) -> Rotation {
        Rotation {
            piece: self.variant,
            from: self.direction,
            to: self.direction + at,
        }
    }

    pub fn rotate(&self, at: Spin) -> Self {
        (*self).tap_mut(|tet| {
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
