#![feature(iterator_try_collect)]

use macroquad::prelude::*;
use selection::Selection;

mod draw;
mod file;
mod selection;
mod state;

#[macroquad::main("Viewtris")]
async fn main() {
    let mut menu = Selection::default();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::O)
            && (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
        {
            if let Ok(new_actions) = file::open_file() {
                menu = new_actions;
            }
        }

        menu.control();
        menu.draw();
        menu.run();

        next_frame().await
    }
}
