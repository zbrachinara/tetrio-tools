#![allow(unused)]

use bsr_tools::{action::Action, tetromino::Spin};

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
            das: 10.,
            arr: 2.,
            sdf: 6.,
            dcd: 1.,
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

/// Holds the entire state of the game. These are:
/// * Board state
/// * Controller state
/// * Future events
/// * Handling/ingame settings
struct Controller<It> {
    events: It,
    board: Board,
    settings: Settings,
    state: State,
}

/// Holds the various states of the controller. Since this replay reader reads frames one-by-one
/// to produce a stream of events, the effects from passive states (such as soft drop) may have
/// been felt multiple times or depend on states which have activated previously (such as hard
/// drop).
#[derive(Default)]
struct State {
    gravity_counter: f32,
    shift_counter: f32,
    shifting: ShiftDirection,

    // button presses
    hard_drop: bool,
    hold: bool,
    rotate_cw: bool,
    rotate_ccw: bool,
    rotate_flip: bool,
}

impl State {
    /// Manages the recorded keypresses and sends commands accordingly to the board. Handles the
    /// logic of DAS and SDF.
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
                Key::Clockwise => { //TODO: Maybe button logic is unnecessary if valid ttrm files validate them
                    if !self.rotate_cw {
                        stream.extend(board.rotate_active(Spin::CW));
                        self.rotate_cw = true;
                    }
                }
                Key::CounterClockwise => {
                    if !self.rotate_ccw {
                        stream.extend(board.rotate_active(Spin::CCW));
                        self.rotate_ccw = true;
                    }
                }
                Key::Flip => {
                    if !self.rotate_flip {
                        stream.extend(board.rotate_active(Spin::Flip));
                        self.rotate_flip = true;
                    }
                }
                Key::Hold => {
                    if !self.hold {
                        stream.extend(board.hold());
                        self.hold = true;
                    }
                }
                Key::HardDrop => {
                    if !self.hard_drop {
                        stream.extend(board.drop_active());
                        self.hard_drop = true;
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
                Key::Clockwise => self.rotate_cw = false,
                Key::CounterClockwise => self.rotate_ccw = false,
                Key::Flip => self.rotate_flip = false,
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
