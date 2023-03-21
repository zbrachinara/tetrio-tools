use macroquad::prelude::*;
pub fn draw_grid(width: usize, height: usize, scale: f32, center_x: f32, center_y: f32) {
    let size = 30. * scale;

    let width_bound_left = center_x - size * width as f32 / 2.;
    let width_bound_right = center_x + size * width as f32 / 2.;
    let height_bound_lower = center_y - size * height as f32 / 2.;
    let height_bound_upper = center_y + size * height as f32 / 2.;

    for col in 0..=width {
        let t = col as f32 * size + width_bound_left;
        draw_line(t, height_bound_lower, t, height_bound_upper, 1.0, WHITE)
    }

    for row in 0..=height {
        let t = row as f32 * size + height_bound_lower;
        draw_line(width_bound_left, t, width_bound_right, t, 1.0, WHITE)
    }
}
