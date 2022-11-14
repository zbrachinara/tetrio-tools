use bsr_tools::tetromino::{Cell, Mino, MinoVariant};
use gridly::{
    prelude::{Grid, GridBounds},
    vector::Vector,
};
use gridly_grids::VecGrid;

use macroquad::prelude::*;

impl From<&MinoVariant> for MinoColor {
    fn from(v: &MinoVariant) -> Self {
        use MinoVariant::*;
        match v {
            L => Self::L,
            J => Self::J,
            T => Self::T,
            Z => Self::Z,
            S => Self::S,
            O => Self::O,
            I => Self::I,
        }
    }
}

impl TryFrom<&Cell> for MinoColor {
    type Error = ();

    fn try_from(value: &Cell) -> Result<Self, Self::Error> {
        match value {
            Cell::Tetromino(tet) => Ok(tet.into()),
            Cell::Garbage => Ok(Self::Gb),
            Cell::Empty => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
#[rustfmt::skip]
#[allow(dead_code)]
enum MinoColor {
    L, J, T, Z, S, O, I, Gb
}

pub struct Board {
    pub cells: VecGrid<Cell>,
    pub active: Mino,
}

impl Board {
    fn enumerated(&self) -> impl Iterator<Item = ((usize, usize), &Cell)> {
        self.cells
            .rows()
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| ((x, y), cell)))
    }
}

pub fn draw_board(board: &Board, legal_region: usize, scale: f32) {
    let size = 30. * scale;

    let Vector { columns, .. } = board.cells.dimensions();
    let origin = (
        screen_width() / 2. - (columns.0 as f32 * size / 2.) as f32,
        screen_height() / 2. + legal_region as f32 * size / 2.,
    );

    for ((x, y), cell) in board.enumerated() {
        if let Ok(color) = MinoColor::try_from(cell) {
            draw_rectangle(
                dbg!(origin.0 + size * x as f32),
                dbg!(origin.1 - size * (y + 1) as f32),
                size,
                size,
                match color {
                    MinoColor::T => PURPLE,
                    MinoColor::L => ORANGE,
                    MinoColor::J => BLUE,
                    MinoColor::S => GREEN,
                    MinoColor::Z => RED,
                    MinoColor::O => YELLOW,
                    MinoColor::I => Color {
                        r: 0.,
                        g: 1.,
                        b: 1.,
                        a: 1.,
                    },
                    MinoColor::Gb => GRAY,
                },
            )
        }
    }
    println!();
}
