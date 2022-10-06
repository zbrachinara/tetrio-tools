use glium::{implement_vertex, Display, DrawError, Frame, Program, VertexBuffer};
use itertools::Itertools;

#[derive(Copy, Clone)]
struct Vertex2 {
    position: [f32; 2],
}

implement_vertex!(Vertex2, position);

const vertex_shader: &'static str = r#"
#version 140

in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const fragment_shader: &'static str = r#"
#version 140

out vec4 color;

void main() {
    color = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;

pub struct DrawProgram {
    program: Program,
    grid: VertexBuffer<Vertex2>,
}

impl DrawProgram {
    pub fn new(display: &Display) -> Self {
        let vbuffer = (0..10)
            .cartesian_product(0..20)
            .flat_map(|(x, y)| {
                [
                    (x, y),
                    (x + 1, y),
                    (x + 1, y),
                    (x + 1, y + 1),
                    (x + 1, y + 1),
                    (x, y + 1),
                    (x, y + 1),
                    (x, y),
                ]
            })
            .map(|(x, y)| Vertex2 {
                position: [x as f32, y as f32],
            })
            .collect::<Vec<_>>();

        Self {
            program: Program::from_source(display, vertex_shader, fragment_shader, None).unwrap(),
            grid: VertexBuffer::immutable(display, &vbuffer).unwrap(),
        }
    }
    pub fn draw_grid(frame: &mut Frame) -> Result<(), DrawError> {
        todo!()
    }
}
