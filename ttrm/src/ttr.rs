use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::{GameType, Replay, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ttr<'a> {
    #[serde(borrow)]
    pub user: User<'a>,
    #[serde(rename = "endcontext")]
    pub end_context: EndContext,
    #[serde(rename = "ts")]
    pub timestamp: &'a str,
    pub data: Replay<'a>,
    #[serde(rename = "gametype")]
    pub game_type: GameType,
    #[serde(rename = "customtype")]
    pub custom_type: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndContext {
    pub seed: u64,
    pub lines: u64,
    pub level_lines_needed: u64,
    pub inputs: u64,
    pub holds: u64,
    pub time: Time,
    pub score: u64,
    #[serde(rename = "zenlevel")]
    pub zen_level: u64,
    #[serde(rename = "zenprogress")]
    pub zen_progress: u64,
    pub level: u64,
    pub combo: u64,
    #[serde(rename = "currentcombopower")]
    pub combo_power: u64,
    #[serde(rename = "topcombo")]
    pub top_combo: u64,
    pub btb: u64,
    #[serde(rename = "topbtb")]
    pub top_b2b: u64,
    #[serde(rename = "currentbtbchainpower")]
    pub current_b2b_power: u64,
    pub tspins: u64,
    #[serde(rename = "piecesplaced")]
    pub pieces_placed: u64,
    pub clears: Clears,
    pub garbage: Garbage,
    pub kills: u64,
    pub finesse: Finesse,
    #[serde(rename = "finalTime")]
    pub final_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub start: f64,
    pub zero: bool,
    pub locked: bool,
    pub prev: u64,
    #[serde(rename = "frameoffset")]
    pub frame_offset: Number, // could possibly be u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clears {
    pub singles: u64,
    pub doubles: u64,
    pub triples: u64,
    pub quads: u64,
    pub realtspins: u64,
    pub minitspins: u64,
    pub minitspinsingles: u64,
    pub tspinsingles: u64,
    pub minitspindoubles: u64,
    pub tspindoubles: u64,
    pub tspintriples: u64,
    pub tspinquads: u64,
    pub allclear: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Garbage {
    pub sent: u64,
    pub received: u64,
    pub attack: u64,
    pub cleared: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Finesse {
    pub combo: u64,
    pub faults: u64,
    pub perfectpieces: u64,
}
