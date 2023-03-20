use macroquad::prelude::*;
use tetrio_replay::viewtris::action::Action;

use crate::draw::{self, board::Board};

#[derive(Default)]
pub struct GameState {
    concurrent_replays: Vec<Replay>,
    frame: u32, // 828 days worth of frames 👍
    /// The time (in macroquad terms) when playing began
    playing_since: Option<f64>,
    unpaused_on_frame: u32,
}

struct Replay {
    board: Board,
    actions: Vec<Action>,
    actions_passed: usize,
}

impl Replay {
    fn with_actions(actions: Vec<Action>) -> Self {
        Self {
            board: Board::empty(),
            actions,
            actions_passed: 0,
        }
    }

    fn is_finished(&self) -> bool {
        self.actions_passed >= self.actions.len()
    }

    fn advance_to_frame(&mut self, new_frame: u32) {
        while let Some(action) = self.actions.get(self.actions_passed) {
            if action.frame > new_frame {
                break;
            }
            self.board.apply_action(&action.kind);
            self.actions_passed += 1;
        }
    }

    pub fn rewind_to_frame(&mut self, new_frame: u32) {
        if self.actions_passed > 0 && self.actions[self.actions_passed - 1].frame > new_frame {
            while self.actions_passed > 0 {
                let action = &self.actions[self.actions_passed - 1];
                self.board.rollback_action(&action.kind);
                if action.frame <= new_frame {
                    break;
                } else {
                    self.actions_passed -= 1;
                }
            }
        }
    }

    pub fn draw(&self) {
        draw::grid::draw_grid(10, 20, 1.0);
        draw::board::draw_board(&self.board, 20, 1.0);
    }
}

impl GameState {
    pub fn with_actions(actions: Vec<Action>) -> Self {
        let mut game_state = Self {
            concurrent_replays: vec![Replay::with_actions(actions)],
            ..Self::default()
        };
        game_state.advance_actions();
        game_state
    }

    pub fn draw(&self) {
        for replay in &self.concurrent_replays {
            replay.draw();
        }
        draw_text(&format!("frame {}", self.frame), 10., 26., 16., WHITE);
        draw_text(
            &format!("in seconds: {:.3}", self.frame as f32 / 60.),
            10.,
            52.,
            16.,
            WHITE,
        );

        // play/pause indicator
        if self.is_paused() {
            draw_rectangle(32.0, 78.0, 16.0, 40.0, WHITE);
            draw_rectangle(56.0, 78.0, 16.0, 40.0, WHITE);
        } else {
            draw_triangle(vec2(32.0, 78.0), vec2(32.0, 118.0), vec2(72.0, 98.0), WHITE);
        }
    }

    pub fn run_player(&mut self) {
        if let Some(playing_since) = self.playing_since {
            let frame = (get_time() - playing_since) * 60.;
            let new_frame = frame.floor() as u32;
            let frame_difference = new_frame - (self.frame - self.unpaused_on_frame);
            for _ in 0..frame_difference {
                self.advance_frame();
            }
            if self.is_finished() {
                self.pause()
            }
        }
    }

    pub fn play(&mut self) {
        self.unpaused_on_frame = self.frame;
        self.playing_since = Some(get_time());
    }

    pub fn pause(&mut self) {
        self.playing_since = None;
    }

    pub fn is_paused(&self) -> bool {
        self.playing_since.is_none()
    }

    pub fn toggle_pause(&mut self) {
        if self.playing_since.is_some() {
            self.pause();
        } else {
            self.play();
        }
    }

    fn is_finished(&self) -> bool {
        self.concurrent_replays
            .iter()
            .all(|replay| replay.is_finished())
    }

    pub fn advance_frame(&mut self) {
        if !self.is_finished() {
            self.frame += 1;
            self.advance_actions();
        }
    }

    fn advance_actions(&mut self) {
        for replay in &mut self.concurrent_replays {
            replay.advance_to_frame(self.frame);
        }
    }

    pub fn rewind_frame(&mut self) {
        if self.frame > 0 {
            self.frame -= 1;

            for replay in &mut self.concurrent_replays {
                replay.rewind_to_frame(self.frame);
            }
        }
    }
}
