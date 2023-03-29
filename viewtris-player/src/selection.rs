use macroquad::prelude::*;

use crate::state::ReplayState;

#[derive(Default)]
pub struct Selection {
    pub replays: Vec<ReplayState>,
    pub camera: Camera2D,
    pub selected: usize,
    pub in_replay: bool,
}

impl Selection {
    pub fn draw(&self) {
        if self.in_replay {
            self.replays[self.selected].draw()
        } else if self.replays.is_empty() {
            draw_text(
                "NO REPLAYS (open a replay with ctrl+O)",
                100.,
                100.,
                32.,
                WHITE,
            )
        } else {
            let w = 400.;
            let x = (screen_width() - w) / 2.;
            let h = 30.;
            let y_root = (screen_height() - self.replays.len() as f32 * h) / 2.;

            // TODO encode and show information about replay states
            for (i, _) in self.replays.iter().enumerate() {
                let y = y_root + i as f32 * h;
                let text = format!("Replay {}", i + 1);
                if i == self.selected {
                    draw_rectangle(x, y, w, h, WHITE);
                    draw_text(text.as_str(), x + 10., y + h / 2., 16., BLACK);
                } else {
                    draw_rectangle_lines(x, y, w, h, 3.0, WHITE);
                    draw_text(text.as_str(), x + 10., y + h / 2., 16., WHITE);
                }
            }

            draw_text("REPLAY SELECTION", x, y_root - 40., 32., WHITE)
        }
    }

    pub fn control(&mut self) {
        if self.in_replay {
            let replay = &mut self.replays[self.selected];

            if is_key_pressed(KeyCode::Period) && replay.is_paused() {
                replay.advance_frame();
            }
            if is_key_pressed(KeyCode::Comma) && replay.is_paused() {
                replay.rewind_frame();
            }

            if is_key_pressed(KeyCode::Space) {
                replay.toggle_pause();
            }

            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                if is_key_pressed(KeyCode::B) {
                    self.in_replay = false;
                }
                if is_key_pressed(KeyCode::R) {
                    replay.reset_to_beginning();
                }
            }
        } else if !self.replays.is_empty() {
            if is_key_pressed(KeyCode::Down) {
                self.selected = (self.selected + 1).clamp(0, self.replays.len() - 1)
            }
            if is_key_pressed(KeyCode::Up) {
                self.selected = self.selected.saturating_sub(1);
            }

            if is_key_pressed(KeyCode::Space)
                || is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::KpEnter)
            {
                self.in_replay = true;
            }
        }
    }

    pub fn run(&mut self) {
        if self.in_replay {
            self.replays[self.selected].run_player();
        }
    }
}
