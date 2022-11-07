use std::{ops::Add, str::FromStr};

use strum::EnumString;
use tap::Tap;

use crate::kick_table::{Positions, ROTATION_TABLE};

impl From<MinoVariant> for Cell {
    fn from(value: MinoVariant) -> Self {
        Self::Tetromino(value)
    }
}

/// The possible states that a cell can take up. A Tetromino cell refers to a filled cell with the
/// color associated with that tetromino
#[derive(Clone, Debug)]
pub enum Cell {
    Tetromino(MinoVariant),
    Garbage,
    Empty,
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
#[derive(Clone)]
pub struct Mino {
    /// The type of mino that is represented (determines its shape)
    pub variant: MinoVariant,
    /// The direction in which the mino is pointing
    pub direction: Direction,
    /// The "center" here is defined by [Positions::tetromino]. It is an arbitrary point which can
    /// be thought of as the "center of rotation", but its most useful property is that it uniquely
    /// determines the absolute position of the tetromino based on its variant and direction.
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
