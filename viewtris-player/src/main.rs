#![feature(iterator_try_collect)]

use std::{fs, os::unix::prelude::OsStrExt};

use itertools::Itertools;
use macroquad::prelude::*;
use selection::Selection;
use state::ReplayState;

mod draw;
mod selection;
mod state;

fn open_file() -> Result<Selection, ()> {
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
                    .map(|actions| Selection {
                        replays: vec![ReplayState::with_actions([actions])],
                        selected: 0,
                        in_replay: true,
                    })
            }
            Some(x) if x.as_bytes() == b"ttrm" => {
                tetrio_replay::ttrm::ttrm_from_slice(buf.as_slice())
                    .ok()
                    .and_then(|ttrm| {
                        let replays = ttrm
                            .data
                            .iter()
                            .filter_map(|player| {
                                player
                                    .replays
                                    .iter()
                                    .map(|replay| tetrio_replay::reconstruct(&replay.events).ok())
                                    .collect::<Option<Vec<_>>>()
                                    .map(ReplayState::with_actions)
                            })
                            .collect_vec();

                        (!replays.is_empty()).then_some(Selection {
                            replays,
                            selected: 0,
                            in_replay: false,
                        })
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
    let mut menu = Selection::default();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::O)
            && (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
        {
            if let Ok(new_actions) = open_file() {
                menu = new_actions;
            }
        }

        menu.control();
        menu.draw();
        menu.run();

        next_frame().await
    }
}
