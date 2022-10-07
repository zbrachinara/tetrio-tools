use bsr_tools::board::{Cell, Mino, MinoVariant};
use glium::{
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    uniform, vertex, Display, DrawError, Frame, Program, Surface, VertexBuffer,
};
use gridly::prelude::{Grid, GridBounds};
use gridly_grids::VecGrid;
use tap::Pipe;

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

#[derive(Copy, Clone, Debug)]
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

pub struct Board {
    pub cells: VecGrid<Cell>,
    pub active: Mino,
}

#[derive(Copy, Clone, Debug)]
pub struct MinoVertex {
    position: [f32; 2],
    color_id: MinoColor,
}

implement_vertex!(MinoVertex, position, color_id);

pub fn board_vertex_buffer(frame: &Display, b: &Board) -> VertexBuffer<MinoVertex> {
    let vbuffer = b
        .cells
        .rows()
        .iter()
        .enumerate()
        .map(|(by, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(bx, elem)| MinoColor::try_from(elem).ok().map(|color| (bx, color)))
                .flat_map(move |(bx, color)| {
                    [
                        // triangle 1
                        ([bx, by], color),
                        ([bx + 1, by], color),
                        ([bx, by + 1], color),
                        // triangle 2
                        ([bx + 1, by + 1], color),
                        ([bx + 1, by], color),
                        ([bx, by + 1], color),
                    ]
                })
                .map(|([px, py], color_id)| MinoVertex {
                    position: [px as f32, py as f32],
                    color_id,
                })
        })
        .flatten()
        // TODO: Chain in the active mino's cells
        .collect::<Vec<_>>();

    VertexBuffer::new(frame, &vbuffer).unwrap()
}
const VERTEX_SHADER: &'static str = r#"
#version 140

in vec2 position;
in uint color_id;
flat out uint color_id_out;

uniform vec2 scale_factor;
uniform vec2 offset;

void main() {
    gl_Position = vec4((position + offset) * scale_factor, 0.0, 1.0);
    color_id_out = color_id;
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 140

flat in uint color_id_out;
out vec4 color;

void main() {
    switch (color_id_out) {
        case 0u: // L piece
            color = vec4(1.0, 0.1, 0.0, 1.0);
            break;
        case 1u: // J piece
            color = vec4(0.0, 0.0, 1.0, 1.0);
            break;
        case 2u: // T piece
            color = vec4(0.5, 0.0, 1.0, 1.0);
            break;
        case 3u: // Z piece
            color = vec4(1.0, 0.0, 0.0, 1.0);
            break;
        case 4u: // S piece
            color = vec4(0.1, 1.0, 0.0, 1.0);
            break;
        case 5u: // O piece
            color = vec4(1.0, 1.0, 0.0, 1.0);
            break;
        case 6u: // I piece
            color = vec4(0.0, 1.0, 1.0, 1.0);
            break;
        case 7u: // garbage piece
            color = vec4(0.6, 0.6, 0.6, 1.0);
            break;
    }
}
"#;

pub struct DrawBoard {
    program: Program,
}

impl DrawBoard {
    pub fn new(dpy: &Display) -> Self {
        Self {
            program: Program::from_source(dpy, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
        }
    }

    pub fn draw_board(
        &self,
        display: &Display,
        frame: &mut Frame,
        board: &Board,
    ) -> Result<(), DrawError> {
        let (win_x, win_y) = frame.get_dimensions().pipe(|(x, y)| (x as f32, y as f32));
        let rect_ratio = win_x / win_y;
        let screen_ratio = 50. / win_x;

        let size = board.cells.dimensions();

        frame.draw(
            &board_vertex_buffer(display, board),
            NoIndices(PrimitiveType::TrianglesList),
            &self.program,
            &uniform! {
                scale_factor: [screen_ratio, screen_ratio * rect_ratio],
                offset: [-(size.columns.0 as f32) / 2., -(size.rows.0 as f32) / 2.],
            },
            &Default::default(),
        )
    }
}
