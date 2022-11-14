use bsr_tools::tetromino::{Cell, Direction, Mino, MinoVariant};
use draw::board::{Board};

use gridly::prelude::Grid;
use gridly_grids::VecGrid;
use macroquad::prelude::*;

mod draw;

#[rustfmt::skip]
const TEST_BOARD : [[Cell; 10]; 20] = {
    use Cell::Garbage as Garbag;
    use Cell::Empty as NoCell;
    use Cell::Tetromino as Tet;
    use MinoVariant::*;
    [
        [Garbag, Garbag, NoCell, Garbag, Garbag, Garbag, Garbag, Garbag, Garbag, Garbag],
        [Tet(J), Tet(J), NoCell, Tet(I), Tet(I), Tet(I), Tet(I), Tet(S), Tet(O), Tet(O)],
        [Tet(J), NoCell, NoCell, NoCell, Tet(Z), Tet(Z), Tet(S), Tet(S), Tet(O), Tet(O)],
        [Tet(J), NoCell, NoCell, Tet(Z), Tet(Z), Tet(L), Tet(S), NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, Tet(L), Tet(L), Tet(L), NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
        [NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell, NoCell],
    ]
};

#[macroquad::main("bsr player")]
async fn main() {

    let board = Board { cells: VecGrid::new_from_rows(TEST_BOARD).unwrap(), active: Mino { variant: MinoVariant::T, direction: Direction::Up, center: (5, 22) } };

    println!("{}", board.cells.display_with(|u| u.clone()));
    println!("{} {}", 
    
        screen_width() / 2. - (10 / 2_isize) as f32,
        screen_height() / 2. - 20 as f32 * 30.0 / 2.,
);

    loop {
        clear_background(BLACK);

        draw_rectangle(0., 0., 40.0, 40.0, WHITE);

        draw::grid::draw_grid(10, 20, 1.0);
        draw::board::draw_board(&board, 20, 1.0);

        next_frame().await
    }
}
