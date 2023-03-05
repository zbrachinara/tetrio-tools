#![allow(unused)]

use bsr_tools::{
    action::{Action, ActionKind},
    tetromino::Spin,
};
use ttrm::event::{Event, EventData, Game, GameOptions, Key, KeyEvent};

use crate::board::Board;

pub struct Settings {
    pub gravity: f64,
    pub gravity_increase: f64,
    pub das: u64,
    pub arr: u64,
    pub sdf: u64,
    pub dcd: u64,
    pub lock_delay: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gravity: 0.01,
            // TODO: Find tetrio defaults for below fields
            gravity_increase: Default::default(),
            das: 100,
            arr: 20,
            sdf: 60,
            dcd: 10,
            lock_delay: 30,
        }
    }
}

impl<'a, 'b> From<&'a GameOptions<'b>> for Settings {
    fn from(options: &'a GameOptions<'b>) -> Self {
        let mut settings = Self {
            gravity: options.gravity,
            gravity_increase: options.gravity_increase.unwrap_or(0.0),
            // das: options
            //     .handling
            //     .map(|h| (h.das * 10.).round() as u64)
            //     .unwrap_or(Settings::default().das),
            // arr: (options.handling.arr * 10.).round() as u64,
            // sdf: options.handling.sdf as u64,
            // dcd: (options.handling.dcd * 10.).round() as u64,
            lock_delay: options.lock_time.unwrap_or(30),
            ..Default::default()
        };

        if let Some(ref handling) = options.handling {
            settings.das = (handling.das * 10.).round() as u64;
            settings.arr = (handling.arr * 10.).round() as u64;
            settings.sdf = handling.sdf as u64;
            settings.dcd = (handling.dcd * 10.).round() as u64;
        }

        settings
    }
}

#[rustfmt::skip]
#[derive(Default)]
pub enum ShiftDirection { #[default] None, Left, Right }

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
pub struct State {
    pub soft_dropping: bool,
    /// The subframe on which either the left or right key was pressed. This should *not* be used in
    /// order to calculate the first shift left or right, but instead to calculate the DAS
    pub shift_began: u64,
    pub last_subframe: u64,
    pub shifting: ShiftDirection,
}

impl State {
    /// Manages the recorded keypresses and sends commands accordingly to the board. Handles the
    /// logic of DAS and SDF.
    fn handle_keys(
        &mut self,
        board: &mut Board,
        settings: &Settings,
        stream: &mut Vec<Action>,
        event: &KeyEvent,
        down: bool,
        frame: u64,
    ) {
        let current_subframe = frame * 10 + (event.subframe.as_f64().unwrap() * 10.).round() as u64;
        stream.extend(board.passive_effects(current_subframe, settings, self));

        if down {
            match event.key {
                // holdable keypresses
                Key::Left => {
                    self.shift_began = current_subframe;
                    stream.extend(board.shift(-1).into_iter().map(|u| u.attach_frame(frame)));
                    self.shifting = ShiftDirection::Left;
                }
                Key::Right => {
                    self.shift_began = current_subframe;
                    stream.extend(board.shift(1).into_iter().map(|u| u.attach_frame(frame)));
                    self.shifting = ShiftDirection::Right;
                }
                Key::SoftDrop => self.soft_dropping = true,
                // single keypresses
                Key::Clockwise => stream.extend(
                    board
                        .rotate_active(Spin::CW)
                        .into_iter()
                        .map(|u| u.attach_frame(frame)),
                ),
                Key::CounterClockwise => stream.extend(
                    board
                        .rotate_active(Spin::CCW)
                        .into_iter()
                        .map(|u| u.attach_frame(frame)),
                ),
                Key::Flip => stream.extend(
                    board
                        .rotate_active(Spin::Flip)
                        .into_iter()
                        .map(|u| u.attach_frame(frame)),
                ),
                Key::Hold => stream.extend(board.hold().map(|u| u.attach_frame(frame))),
                Key::HardDrop => stream.extend(
                    board
                        .drop_active()
                        .iter()
                        .map(|u| u.clone().attach_frame(frame)),
                ),
            }
        } else {
            match event.key {
                // holdable keypresses
                Key::Left | Key::Right => self.shifting = ShiftDirection::None,
                Key::SoftDrop => self.soft_dropping = false,
                // single keypresses
                _ => (),
            }
        }

        self.last_subframe = current_subframe;
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
                    self.state.handle_keys(
                        &mut self.board,
                        &self.settings,
                        &mut stream,
                        key_event,
                        true,
                        event.frame,
                    );
                }
                EventData::KeyUp { ref key_event } => self.state.handle_keys(
                    &mut self.board,
                    &self.settings,
                    &mut stream,
                    key_event,
                    false,
                    event.frame,
                ),
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
