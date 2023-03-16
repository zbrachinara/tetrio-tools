use macroquad::prelude::*;
use tetrio_replay::viewtris::action::Action;

use crate::draw::{self, board::Board};

pub struct GameState {
    board: Board,
    actions: Vec<Action>,
    actions_passed: usize,
    frame: u32, // 828 days worth of frames 👍
    /// The time (in macroquad terms) when playing began
    playing_since: Option<f64>,
}

impl GameState {
    pub fn empty() -> Self {
        Self {
            board: Board::empty(),
            actions: vec![],
            actions_passed: 0,
            frame: 0,
            playing_since: None,
        }
    }

    pub fn with_actions(actions: Vec<Action>) -> Self {
        let mut game_state = Self {
            board: Board::empty(),
            actions,
            actions_passed: 0,
            frame: 0,
            playing_since: None,
        };
        game_state.advance_actions();
        game_state
    }

    pub fn draw(&self) {
        draw::grid::draw_grid(10, 20, 1.0);
        draw::board::draw_board(&self.board, 20, 1.0);
        draw_text(&format!("frame {}", self.frame), 10., 26., 16., WHITE);
        draw_text(
            &format!("in seconds: {:.3}", self.frame as f32 / 60.),
            10.,
            52.,
            16.,
            WHITE,
        );
    }

    pub fn run_player(&mut self) {
        if let Some(playing_since) = self.playing_since {
            let frame = (get_time() - playing_since) * 60.;
            let new_frame = frame.floor() as u32;
            let frame_difference = new_frame - self.frame;
            for _ in 0..frame_difference {
                self.advance_frame();
            }
        }
    }

    pub fn play(&mut self) {
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
        self.actions_passed >= self.actions.len()
    }

    pub fn advance_frame(&mut self) {
        if !self.is_finished() {
            self.frame += 1;
            self.advance_actions();
        }
    }

    fn advance_actions(&mut self) {
        while let Some(action) = self.actions.get(self.actions_passed) {
            if action.frame > self.frame as u64 {
                break;
            }
            self.board.apply_action(&action.kind);
            self.actions_passed += 1;
        }
    }

    pub fn rewind_frame(&mut self) {
        if self.frame > 0 {
            self.frame -= 1;

            if self.actions_passed > 0
                && self.actions[self.actions_passed - 1].frame > self.frame as u64
            {
                while self.actions_passed > 0 {
                    let action = &self.actions[self.actions_passed - 1];
                    self.board.rollback_action(&action.kind);
                    if action.frame <= self.frame as u64 {
                        break;
                    } else {
                        self.actions_passed -= 1;
                    }
                }
            }
        }
    }
}
