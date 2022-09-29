#![allow(unused)]

use crate::{board::{Cell, Mino}, data::event::Event};

pub enum Action {
    Garbage { column: u8, height: u8 },
    Reposition { piece: Mino },
    LineClear { line: u8 },
    Cell { position: (u8, u8), kind: Cell },
}


fn reconstruct<'a> (event_stream: Vec<Event<'a>>) -> Vec<Action> {
    todo!()
}