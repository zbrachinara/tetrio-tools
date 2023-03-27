use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::{event, GameType};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ttrm<'a> {
    #[serde(rename = "_id")]
    pub id: &'a str,
    pub back: &'a str,
    pub data: Vec<ReplaySet<'a>>,
    #[serde(rename = "forcestyle")]
    pub force_style: &'a str,
    #[serde(rename = "gametype")]
    pub game_type: GameType,
    #[serde(rename = "ismulti")]
    pub is_multi: bool,
    #[serde(rename = "shortid")]
    pub short_id: &'a str,
    #[serde(rename = "ts")]
    pub timestamp: &'a str,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplaySet<'a> {
    #[serde(rename = "board", borrow)]
    pub boards: Vec<Board<'a>>,
    pub replays: Vec<Replay<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Board<'a> {
    pub active: bool,
    pub success: bool,
    #[serde(borrow)]
    pub user: User<'a>,
    pub winning: Number,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User<'a> {
    #[serde(rename = "_id")]
    pub user_id: &'a str,
    pub username: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Replay<'a> {
    // pub events: Vec<Value>,
    #[serde(borrow)]
    pub events: Vec<event::Event<'a>>,
    pub frames: Number,
}
