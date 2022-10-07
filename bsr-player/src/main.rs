use std::time::{Duration, Instant};

use bsr_tools::{board::{Board, Mino, MinoVariant, Direction, Cell}, rng::PieceQueue};
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    }, Program, Surface,
};
use gridly_grids::VecGrid;

use crate::board::board_vertex_buffer;

mod draw;
mod board;


fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Hello world!");
    let windowed_context = ContextBuilder::new();

    let display = glium::Display::new(wb, windowed_context, &el).unwrap();

    let vertex_shader = r#"
#version 140

in vec2 position;
in uint color_id;
flat out uint color_id_out;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    color_id_out = color_id;
}
"#;

    let fragment_shader = r#"
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
    }
}
"#;

    let shader = Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();
    let draw_grid = draw::grid::DrawProgram::new(&display, (10, 20));
    let draw_board = board::DrawBoard::new(&display);



    use MinoVariant::*;
    use Cell::Tetromino as Tet;
    use Cell::Garbage as Gb;
    use Cell::None as Nn;
    let example_board = Board {
        cells: VecGrid::new_from_rows([
            [Tet(J), Tet(J), Nn, Tet(I), Tet(I), Tet(I), Tet(I), Tet(S), Tet(O), Tet(O)]
        ]).unwrap(),
        queue: PieceQueue::meaningless(),
        active: Mino {
            variant: MinoVariant::T,
            rotation_state: Direction::Up,
            position: (5, 22),
        },
    };

    el.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        draw_grid.draw_grid(&mut target).unwrap();
        draw_board.draw_board(&display, &mut target, &example_board).unwrap();
        // target.draw(
        //     &vbuffer,
        //     &NoIndices(PrimitiveType::TrianglesList),
        //     &shader,
        //     &EmptyUniforms,
        //     &Default::default(),
        // ).unwrap();

        target.finish().unwrap();

        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);

        *control_flow = ControlFlow::WaitUntil(next_frame_time);
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
