use std::fs;

use draw::board::Board;

use macroquad::prelude::*;

mod draw;

#[macroquad::main("bsr player")]
async fn main() {
    let mut board = Board::empty();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::O)
            && (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
        {
            if let Some(fi) = rfd::FileDialog::new().pick_file() {
                board = Board::empty();
                let buf = fs::read_to_string(fi);
                println!("{buf:?}")
            }
        }

        draw::grid::draw_grid(10, 20, 1.0);
        draw::board::draw_board(&board, 20, 1.0);

        next_frame().await
    }
}
