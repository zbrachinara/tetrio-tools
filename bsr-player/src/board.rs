use glium::{vertex, implement_vertex};
use gridly_grids::VecGrid;

#[derive(Copy, Clone)]
#[repr(u32)]
#[rustfmt::skip]
#[allow(dead_code)]
enum MinoColor {
    L, J, T, Z, S, O, I
}

unsafe impl vertex::Attribute for MinoColor {
    fn get_type() -> vertex::AttributeType {
        vertex::AttributeType::U32
    }
}

#[derive(Copy, Clone)]
struct MinoVertex {
    position: [f32; 2],
    color_id: MinoColor,
}

implement_vertex!(MinoVertex, position, color_id);

pub struct Board {
    grid: VecGrid<MinoColor>,
    hold: MinoColor,
}