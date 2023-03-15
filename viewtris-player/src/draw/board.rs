use itertools::Itertools;
use tetrio_replay::viewtris::{
    action::ActionKind,
    tetromino::{Cell, Direction, Mino, MinoVariant},
};

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

impl From<MinoVariant> for MinoColor {
    fn from(value: MinoVariant) -> Self {
        Self::from(&value)
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
enum MinoColor {
    L, J, T, Z, S, O, I, Gb
}

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub active: Option<Mino>,
    pub hold: Option<MinoVariant>, // TODO draw
}

impl Board {
    fn enumerated(&self) -> impl Iterator<Item = ((usize, usize), &Cell)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| ((x, y), cell)))
    }

    pub fn empty() -> Self {
        Self {
            cells: (0..20).map(|_| vec![Cell::Empty; 10]).collect_vec(),
            active: None,
            hold: None,
        }
    }

    pub fn apply_action(&mut self, action: &ActionKind) {
        match action {
            ActionKind::Garbage { column, height } => todo!(),
            ActionKind::Reposition { piece } => self.active = Some(*piece),
            ActionKind::LineClear { row } => {
                let row = *row as usize;
                self.cells[row].iter_mut().for_each(|it| *it = Cell::Empty);
                self.cells[row..].rotate_left(1);
            }
            ActionKind::Cell {
                position: (x, y),
                kind,
            } => self.cells[*y as usize][*x as usize] = *kind,
            ActionKind::Hold => {
                let active = std::mem::take(&mut self.active).map(|u| u.variant);
                if let Some(replacing_active) = std::mem::replace(&mut self.hold, active) {
                    self.active = Some(replacing_active.into())
                }
            }
        }
    }
}

fn draw_cell(
    (root_x, root_y): (f32, f32),
    (pos_x, pos_y): (i32, i32),
    color: MinoColor,
    size: f32,
) {
    draw_rectangle(
        root_x + size * pos_x as f32,
        root_y - size * (pos_y + 1) as f32,
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
    );
}

pub fn draw_board(board: &Board, legal_region: usize, scale: f32) {
    let cell_size = 30. * scale;

    let columns = board.cells[0].len();
    let origin = (
        screen_width() / 2. - (columns as f32 * cell_size / 2.),
        screen_height() / 2. + legal_region as f32 * cell_size / 2.,
    );

    for ((x, y), cell) in board.enumerated() {
        if let Ok(color) = MinoColor::try_from(cell) {
            draw_cell(origin, (x as i32, y as i32), color, cell_size)
        }
    }

    if let Some(active) = board.active {
        for (pos_x, pos_y) in active.position().0 {
            draw_cell(
                origin,
                (pos_x as i32, pos_y as i32),
                active.variant.into(),
                cell_size,
            )
        }
    }

    if let Some(held) = board.hold {
        let held_piece = Mino {
            variant: held,
            direction: Direction::Up,
            coord: (0, 0),
        };
        let new_origin = (origin.0 - cell_size * 5., origin.1 - cell_size * 17.);
        for (pos_x, pos_y) in held_piece.position().0 {
            draw_cell(
                new_origin,
                (pos_x as i32, pos_y as i32),
                held.into(),
                cell_size,
            )
        }
    }
}
