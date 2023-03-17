use std::{fs, os::unix::prelude::OsStrExt};

use macroquad::prelude::*;
use state::GameState;
use tetrio_replay::viewtris::action::Action;

mod draw;
mod state;

fn open_file() -> Result<Vec<Action>, ()> {
    rfd::FileDialog::new()
        .pick_file()
        .and_then(|fi| {
            fs::read(fi.clone())
                .map(|buf| (buf, fi.extension().map(|str| str.to_os_string())))
                .ok()
        })
        .and_then(|(buf, extension)| match extension {
            Some(x) if x.as_bytes() == b"ttr" => {
                tetrio_replay::ttrm::ttr_from_slice(buf.as_slice())
                    .ok()
                    .and_then(|ttr| tetrio_replay::reconstruct(ttr.data.events.as_slice()).ok())
            }
            Some(x) if x.as_bytes() == b"ttrm" => {
                tetrio_replay::ttrm::ttrm_from_slice(buf.as_slice())
                    .ok()
                    .and_then(|ttrm| {
                        tetrio_replay::reconstruct(ttrm.data[0].replays[0].events.as_slice()).ok()
                    })
            }
            _ => {
                eprintln!("Unknown file type, this player only expects ttr or ttrm ");
                None
            }
        })
        .ok_or(())
}

#[macroquad::main("Viewtris")]
async fn main() {
    let mut game_state = GameState::default();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::O)
            && (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
        {
            if let Ok(new_actions) = open_file() {
                game_state = GameState::with_actions(new_actions)
            }
        }

        if is_key_pressed(KeyCode::Period) && game_state.is_paused() {
            game_state.advance_frame();
        }
        if is_key_pressed(KeyCode::Comma) && game_state.is_paused() {
            game_state.rewind_frame();
        }

        if is_key_pressed(KeyCode::Space) {
            game_state.toggle_pause();
        }

        game_state.draw();
        game_state.run_player();

        next_frame().await
    }
}
