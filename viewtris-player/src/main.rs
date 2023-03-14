use std::{fs, os::unix::prelude::OsStrExt};

use draw::board::Board;

use macroquad::prelude::*;
use tetrio_replay::viewtris::action::Action;

mod draw;

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

#[macroquad::main("bsr player")]
async fn main() {
    let mut board = Board::empty();
    let mut actions = Vec::<Action>::new();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::O)
            && (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
        {
            board = Board::empty();
            if let Ok(new_actions) = open_file() {
                actions = new_actions;
            }
        }

        draw::grid::draw_grid(10, 20, 1.0);
        draw::board::draw_board(&board, 20, 1.0);

        next_frame().await
    }
}
