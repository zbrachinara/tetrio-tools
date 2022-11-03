#![allow(unused)]

use bsr_tools::action::Action;

use crate::{
    board::Board,
    data::event::{Event, EventData, GameOptions, Key, KeyEvent},
};

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

#[rustfmt::skip]
enum ShiftDirection { None, Left, Right }

impl Default for ShiftDirection {
    fn default() -> Self {
        ShiftDirection::None
    }
}

struct Controller<It> {
    events: It,
    board: Board,
    settings: Settings,
    state: State,
}

#[derive(Default)]
struct State {
    gravity_counter: f32,
    shift_counter: f32,
    shifting: ShiftDirection,

    // button presses
    hard_drop: bool,
    hold: bool,
    rotate: bool, //TODO: Find out if rotation keyup/down events are individually handled
}

impl State {
    fn handle_keys(
        &mut self,
        board: &mut Board,
        stream: &mut Vec<Action>,
        event: &KeyEvent,
        down: bool,
    ) {
        if down {
            match event.key {
                // holdable keypresses
                Key::Left => todo!(),
                Key::Right => todo!(),
                Key::SoftDrop => todo!(),
                // single keypresses
                Key::Clockwise => todo!(),
                Key::CounterClockwise => todo!(),
                Key::Flip => todo!(),
                Key::Hold => {
                    if !self.hold {
                        stream.extend(board.hold());
                        self.hold = true;
                    }
                }
                Key::HardDrop => {
                    if !self.hard_drop {
                        todo!("perform hard drop");
                    }
                }
            }
        } else {
            match event.key {
                // holdable keypresses
                Key::Left => todo!(),
                Key::Right => todo!(),
                Key::SoftDrop => todo!(),
                // single keypresses
                Key::Clockwise => todo!(),
                Key::CounterClockwise => todo!(),
                Key::Flip => todo!(),
                Key::Hold => self.hold = false,
                Key::HardDrop => self.hard_drop = false,
            }
        }
    }
}

impl<'a, It> Controller<It>
where
    It: Iterator<Item = &'a Event<'a>>,
{
    /// Creates a controller from a series of tetrio events
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
                board: Board::new(options.seed, &board.board),
                settings: options.into(),
                state: State::default(),
            }),
            _ => unreachable!(),
        }
    }

    fn stream(mut self) -> Result<Vec<Action>, String> {
        let mut stream = Vec::new();

        let mut initial_frame = 0;
        self.events.for_each(|event| {
            let frames_passed = event.frame - initial_frame;
            initial_frame = event.frame;

            match event.data {
                EventData::Start => unreachable!(),
                EventData::Full { .. } => (),
                EventData::Targets => (),
                EventData::KeyDown { ref key_event } => {
                    self.state
                        .handle_keys(&mut self.board, &mut stream, key_event, true);
                }
                EventData::KeyUp { ref key_event } => {
                    self.state
                        .handle_keys(&mut self.board, &mut stream, key_event, false)
                }
                EventData::InGameEvent { ref event } => todo!(),
                EventData::End => todo!(),
            }

            unimplemented!("game parsing")
        });

        Ok(stream)
    }
}

fn reconstruct<'a>(event_stream: &Vec<Event<'a>>) -> Result<Vec<Action>, String> {
    Controller::read_game(event_stream.iter())?.stream()
}
