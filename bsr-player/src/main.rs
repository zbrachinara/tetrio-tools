use std::time::{Duration, Instant};

use bsr_tools::{
    board::{Board, Cell, Direction, Mino, MinoVariant},
    rng::PieceQueue,
};
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    Surface,
};
use gridly_grids::VecGrid;

mod board;
mod draw;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Hello world!");
    let windowed_context = ContextBuilder::new();

    let display = glium::Display::new(wb, windowed_context, &el).unwrap();

    let draw_grid = draw::grid::DrawProgram::new(&display, (10, 20));
    let draw_board = board::DrawBoard::new(&display);

    use Cell::Garbage as Gb;
    use Cell::None as Nn;
    use Cell::Tetromino as Tet;
    use MinoVariant::*;
    let example_board = Board {
        cells: VecGrid::new_from_rows([[
            Tet(J),
            Tet(J),
            Nn,
            Tet(I),
            Tet(I),
            Tet(I),
            Tet(I),
            Tet(S),
            Tet(O),
            Tet(O),
        ]])
        .unwrap(),
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
        draw_board
            .draw_board(&display, &mut target, &example_board)
            .unwrap();
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
