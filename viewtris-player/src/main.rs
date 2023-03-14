use std::fs;

use draw::board::Board;
// use tetrio_replay::viewtris::tetromino::{Cell, Direction, Mino, MinoVariant};

// use gridly_grids::VecGrid;
use macroquad::prelude::*;

mod draw;

// #[rustfmt::skip]
// const TEST_BOARD : [[Cell; 10]; 20] = {
//     use Cell::Garbage as Garbag;
//     use Cell::Empty as NoCell;
//     use Cell::Tetromino as Tet;
//     use MinoVariant::*;
//     [
//         [Garbag, Garbag, NoCell, Garbag, Garbag, Garbag, Garbag, Garbag, Garbag, Garbag],
//         [Tet(J), Tet(J), NoCell, Tet(I), Tet(I), Tet(I), Tet(I), Tet(S), Tet(O), Tet(O)],
//         [Tet(J), NoCell, NoCell, NoCell, Tet(Z), Tet(Z), Tet(S), Tet(S), Tet(O), Tet(O)],
//         [Tet(J), NoCell, NoCell, Tet(Z), Tet(Z), Tet(L), Tet(S), NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, Tet(L), Tet(L), Tet(L), NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//         [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
//     ]
// };

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
