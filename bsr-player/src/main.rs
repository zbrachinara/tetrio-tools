use bsr_tools::tetromino::{Cell, Direction, Mino, MinoVariant};
use draw::board::Board;

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
    let board = Board {
        cells: VecGrid::new_from_rows(TEST_BOARD).unwrap(),
        active: Mino {
            variant: MinoVariant::T,
            direction: Direction::Up,
            coord: (5, 22),
        },
    };

    loop {
        clear_background(BLACK);

        draw::grid::draw_grid(10, 20, 1.0);
        draw::board::draw_board(&board, 20, 1.0);

        next_frame().await
    }
}
