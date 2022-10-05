use std::time::{Duration, Instant};

use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    uniforms::EmptyUniforms,
    Program, Surface, VertexBuffer,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Hello world!");
    let windowed_context = ContextBuilder::new();

    let display = glium::Display::new(wb, windowed_context, &el).unwrap();

    let vertex_shader = r#"
#version 140

in vec2 position;
out vec4 color_id;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    color_id = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

    let fragment_shader = r#"
#version 140

in vec4 color_id;
out vec4 color;

void main() {
    color = color_id;
}
"#;

    let shader = Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = [vertex1, vertex2, vertex3];

    let vbuffer = VertexBuffer::new(&display, &shape).unwrap();

    el.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(
            &vbuffer,
            &NoIndices(PrimitiveType::TrianglesList),
            &shader,
            &EmptyUniforms,
            &Default::default(),
        ).unwrap();

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
