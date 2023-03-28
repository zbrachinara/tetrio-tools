use ttrm::{
    event::{Event, EventData, EventFull, Game, Key, KeyEvent},
    GameType,
};
use viewtris::{action::Action, tetromino::Spin};

use crate::board::Board;

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
    // settings: Settings,
    state: State,
    stream: Vec<Action>,
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
    pub shift_began: u32,
    pub last_subframe: u32,
    pub shifting: ShiftDirection,
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
        frame: u32,
    ) {
        let current_subframe = frame * 10 + (event.subframe.as_f64().unwrap() * 10.).round() as u32;
        stream.extend(board.passive_effects(current_subframe + 1, self));

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
                Key::Hold => stream.extend(board.hold().into_iter().map(|u| u.attach_frame(frame))),
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
    fn read_game(mut game: It, game_type: GameType) -> Result<Self, &'static str> {
        loop {
            let next = game.next();

            match next {
                Some(Event {
                    data: EventData::Full { data },
                    ..
                }) => {
                    let EventFull {
                        ref options,
                        game: Game { ref board, .. },
                        ..
                    } = **data;
                    let (board, stream) =
                        Board::new(options.seed, game_type, options.into(), board);
                    break Some(Self {
                        events: game,
                        board,
                        state: State::default(),
                        stream,
                    });
                }
                None => break None,
                _ => continue, // keep searching for full data
            }
        }
        .ok_or("could not find full data to extract initial game state from")
    }

    fn stream(mut self) -> Result<Vec<Action>, String> {
        let mut initial_frame = 0;
        self.events.for_each(|event| {
            initial_frame = event.frame;

            match event.data {
                EventData::Start {} => (),
                EventData::Full { .. } => (),
                EventData::Targets { .. } => (),
                EventData::KeyDown { ref key_event } => {
                    self.state.handle_keys(
                        &mut self.board,
                        // &self.settings,
                        &mut self.stream,
                        key_event,
                        true,
                        event.frame,
                    );
                }
                EventData::KeyUp { ref key_event } => self.state.handle_keys(
                    &mut self.board,
                    &mut self.stream,
                    key_event,
                    false,
                    event.frame,
                ),
                EventData::InGameEvent {
                    event: ref ingame_event,
                } => self
                    .board
                    .acknowledge_garbage(&ingame_event.data.data, event.frame),
                EventData::End {} => (),
            }
        });

        Ok(self.stream)
    }
}

pub fn reconstruct(game_type: GameType, event_stream: &[Event]) -> Result<Vec<Action>, String> {
    Controller::read_game(event_stream.iter(), game_type)?.stream()
}
