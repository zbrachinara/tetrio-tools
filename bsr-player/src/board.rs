use bsr_tools::board::{Board, Cell, MinoVariant};
use glium::{implement_vertex, vertex, VertexBuffer, Display};
use gridly::prelude::Grid;

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
            Cell::None => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[rustfmt::skip]
#[allow(dead_code)]
enum MinoColor {
    L, J, T, Z, S, O, I, Gb
}

unsafe impl vertex::Attribute for MinoColor {
    fn get_type() -> vertex::AttributeType {
        vertex::AttributeType::U32
    }
}

#[derive(Copy, Clone)]
pub struct MinoVertex {
    position: [f32; 2],
    color_id: MinoColor,
}

implement_vertex!(MinoVertex, position, color_id);

pub fn board_vertex_buffer(frame: &Display, b: Board) -> VertexBuffer<MinoVertex> {
    let vbuffer = b.cells
        .rows()
        .iter()
        .enumerate()
        .map(|(bx, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(by, elem)| MinoColor::try_from(elem).ok().map(|color| (by, color)))
                .flat_map(move |(by, color)| [
                    // triangle 1
                    ([bx, by], color), ([bx + 1, by], color), ([by + 1, bx], color),
                    // triangle 2
                    ([bx + 1, by + 1], color), ([bx + 1, by], color), ([bx, by + 1], color),
                ])
                .map(|([px, py], color_id)| {
                    MinoVertex {position: [px as f32, py as f32], color_id}
                })
        })
        .flatten()
        .collect::<Vec<_>>();

        VertexBuffer::new(frame, &vbuffer).unwrap()
}
