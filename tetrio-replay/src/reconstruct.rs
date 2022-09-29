#![allow(unused)]

use crate::{
    board::{Cell, Mino},
    data::event::{Event, GameOptions},
};

pub enum Action {
    Garbage { column: u8, height: u8 },
    Reposition { piece: Mino },
    LineClear { line: u8 },
    Cell { position: (u8, u8), kind: Cell },
}

struct Settings {
    gravity: f64,
    gravity_increase: f64,
    das: f64,
    arr: f64,
    sdf: f64,
    dcd: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gravity: 0.01,
            // TODO: Find tetrio defaults for below fields
            gravity_increase: Default::default(),
            das: Default::default(),
            arr: Default::default(),
            sdf: Default::default(),
            dcd: Default::default(),
        }
    }
}

impl<'a> From<GameOptions<'a>> for Settings {
    fn from(options: GameOptions) -> Self {
        Self {
            gravity: options.gravity,
            gravity_increase: options.gravity_increase,
            das: options.handling.das,
            arr: options.handling.arr,
            sdf: options.handling.sdf,
            dcd: options.handling.dcd,
        }
    }
}

#[derive(Default)]
struct Controller {
    settings: Settings,
    gravity_counter: f32,
    shift_counter: f32,
}

impl<'a> From<GameOptions<'a>> for Controller {
    fn from(options: GameOptions<'a>) -> Self {
        Self {
            settings: options.into(),
            ..Default::default()
        }
    }
}

impl Controller {
    fn stream_game<'a>(&mut self, game: &Vec<Event<'a>>) -> Result<Vec<Action>, String> {
        todo!()
    }
}

fn reconstruct<'a>(event_stream: &Vec<Event<'a>>) -> Result<Vec<Action>, String> {
    Controller::default().stream_game(&event_stream)
}
