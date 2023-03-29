use std::{fs, os::unix::prelude::OsStrExt};

use itertools::Itertools;
use macroquad::prelude::*;

fn screen_rect() -> Rect {
    Rect::new(0., 0., screen_width(), screen_height())
}

use crate::{selection::Selection, state::ReplayState};

pub fn open_file() -> Result<Selection, ()> {
    rfd::FileDialog::new()
        .pick_file()
        .and_then(|fi| {
            fs::read(fi.clone())
                .map(|buf| (buf, fi.extension().map(|str| str.to_os_string())))
                .ok()
        })
        .and_then(|(buf, extension)| match extension {
            Some(x) if x.as_bytes() == b"ttr" => read_ttr(buf.as_slice()),
            Some(x) if x.as_bytes() == b"ttrm" => read_ttrm(buf.as_slice()),
            _ => {
                eprintln!("Unknown file type, this player only expects ttr or ttrm ");
                None
            }
        })
        .ok_or(())
}

fn read_ttr(buf: &[u8]) -> Option<Selection> {
    tetrio_replay::ttrm::ttr_from_slice(buf)
        .ok()
        .and_then(|ttr| tetrio_replay::reconstruct(ttr.game_type, ttr.data.events.as_slice()).ok())
        .map(|actions| Selection {
            replays: vec![ReplayState::with_actions([actions])],
            camera: Camera2D::from_display_rect(screen_rect()),
            selected: 0,
            in_replay: true,
        })
}

fn read_ttrm(buf: &[u8]) -> Option<Selection> {
    tetrio_replay::ttrm::ttrm_from_slice(buf)
        .ok()
        .and_then(|ttrm| {
            let replays = ttrm
                .data
                .iter()
                .filter_map(|player| {
                    player
                        .replays
                        .iter()
                        .map(|replay| {
                            tetrio_replay::reconstruct(ttrm.game_type, &replay.events).ok()
                        })
                        .collect::<Option<Vec<_>>>()
                        .map(ReplayState::with_actions)
                })
                .collect_vec();

            (!replays.is_empty()).then_some(Selection {
                replays,
                camera: Camera2D::from_display_rect(screen_rect()),
                selected: 0,
                in_replay: false,
            })
        })
}
