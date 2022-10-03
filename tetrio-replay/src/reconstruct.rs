#![allow(unused)]

use crate::{
    board::{Board, Cell, Mino},
    data::event::{Event, EventData, GameOptions},
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

impl<'a, 'b> From<&'a GameOptions<'b>> for Settings {
    fn from(options: &'a GameOptions<'b>) -> Self {
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

// #[derive(Default)]
struct Controller<It> {
    events: It,
    board: Board,
    settings: Settings,
    gravity_counter: f32,
    shift_counter: f32,
}

impl<'a, It> Controller<It>
where
    It: Iterator<Item = &'a Event<'a>>,
{
    fn read_game(mut game: It) -> Result<Self, &'static str> {
        let pregame_data = loop {
            let next = game.next();
            match next {
                Some(Event {
                    data: EventData::Full { .. },
                    ..
                }) => break next,
                None => break None,
                _ => continue,
            }
        }
        .ok_or("could not find full data to extract initial game state from")?;

        match pregame_data {
            Event {
                data:
                    EventData::Full {
                        options,
                        game: board,
                        ..
                    },
                ..
            } => Ok(Self {
                events: game,
                board: Board::new(options.seed, board),
                settings: options.into(),
                gravity_counter: 0.,
                shift_counter: 0.,
            }),
            _ => unreachable!(),
        }
    }

    fn stream(self) -> Result<Vec<Action>, String> {
        todo!()
    }
}

fn reconstruct<'a>(event_stream: &Vec<Event<'a>>) -> Result<Vec<Action>, String> {
    Controller::read_game(event_stream.iter())?.stream()
}
