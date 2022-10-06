use crate::board::{Mino, Cell};



pub enum Action {
    Garbage { column: u8, height: u8 },
    Reposition { piece: Mino },
    LineClear { line: u8 },
    Cell { position: (u8, u8), kind: Cell },
}