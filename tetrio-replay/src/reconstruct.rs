#![allow(unused)]

use bsr_tools::{action::Action, tetromino::Spin};
use ttrm::event::{Event, EventData, Game, GameOptions, Key, KeyEvent};

use crate::board::Board;

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
#[derive(Default)]
enum ShiftDirection { #[default] None, Left, Right }

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
                Key::Clockwise => stream.extend(board.rotate_active(Spin::CW)),
                Key::CounterClockwise => stream.extend(board.rotate_active(Spin::CCW)),
                Key::Flip => stream.extend(board.rotate_active(Spin::Flip)),
                Key::Hold => stream.extend(board.hold()),
                Key::HardDrop => stream.extend(board.drop_active()),
            }
        } else {
            match event.key {
                // holdable keypresses
                Key::Left => todo!(),
                Key::Right => todo!(),
                Key::SoftDrop => todo!(),
                // single keypresses
                _ => (),
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
        loop {
            let next = game.next();

            match next {
                Some(Event {
                    data:
                        EventData::Full {
                            options,
                            game: Game { board, .. },
                            ..
                        },
                    ..
                }) => {
                    break Some(Self {
                        events: game,
                        board: Board::new(options.seed, board),
                        settings: options.into(),
                        state: State::default(),
                    })
                }
                None => break None,
                _ => continue, // keep searching for full data
            }
        }
        .ok_or("could not find full data to extract initial game state from")
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

fn reconstruct(event_stream: &[Event]) -> Result<Vec<Action>, String> {
    Controller::read_game(event_stream.iter())?.stream()
}
