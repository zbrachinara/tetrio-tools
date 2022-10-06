use std::time::{Duration, Instant};

use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    implement_vertex,
    vertex, Program, Surface,
};

mod draw;

#[derive(Copy, Clone)]
#[repr(u32)]
#[rustfmt::skip]
#[allow(dead_code)]
enum MinoColors {
    L, J, T, Z, S, O, I
}

unsafe impl vertex::Attribute for MinoColors {
    fn get_type() -> vertex::AttributeType {
        vertex::AttributeType::U32
    }
}

#[derive(Copy, Clone)]
struct MinoVertex {
    position: [f32; 2],
    color_id: MinoColors,
}

implement_vertex!(MinoVertex, position, color_id);

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

    el.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        draw_grid.draw_grid(&mut target).unwrap();
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
