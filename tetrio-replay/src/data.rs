use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Debug, Serialize, Deserialize)]
pub struct TTRM<'a> {
    #[serde(rename = "_id")]
    pub id: &'a str,
    pub back: &'a str,
    pub data: Vec<ReplaySet<'a>>,
    #[serde(rename = "forcestyle")]
    pub force_style: &'a str,
    pub gametype: &'a str,
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

pub mod event {
    use serde::{Deserialize, Serialize};
    use serde_json::{Number, Value};
    use tagged_hybrid::hybrid_tagged;

    #[hybrid_tagged(
        fields = {frame: Number},
        tag = "type",
        struct_attrs = { #[derive(Debug)]}
    )]
    #[serde(rename_all = "lowercase")]
    pub enum Event<'a> {
        Start,
        Full {
            #[serde(rename = "aggregatestats")]
            aggregate_stats: AggregateStats,
            assumptions: Value,
            fire: Number,
            game: Game<'a>,
            #[serde(rename = "gameoverreason")]
            game_over_reason: Option<&'a str>,
            killer: Killer<'a>,
            options: GameOptions<'a>,
            replay: Value, //TODO: gonna have to see what these structs mean
            source: Value,
            stats: Value,
            successful: bool,
            targets: Vec<Value>, // TODO: probably string, but have to check
        },
        Targets,
        KeyDown {
            #[serde(flatten, borrow)]
            key_event: KeyEvent<'a>,
        },
        KeyUp {
            #[serde(flatten, borrow)]
            key_event: KeyEvent<'a>,
        },
        #[serde(rename = "ige")]
        InGameEvent {
            #[serde(flatten)]
            event: InteractionContainer,
        },
        End,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct InteractionContainer {
        pub id: Number,
        pub frame: Number,
        pub data: Interaction,
    }

    #[hybrid_tagged(
        fields = {
            sender: String,
            sent_frame: Number,
            cid: Number,
        },
        tag = "type",
        struct_attrs = { #[derive(Debug)]}
    )]
    #[serde(rename_all = "snake_case")]
    pub enum Interaction {
        InteractionDo {
            #[serde(flatten)]
            data: Garbage,
        },
        InteractionConfirm {
            #[serde(flatten)]
            data: Garbage,
        },
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Garbage {
        pub amt: Number,
        pub x: Number,
        pub y: Number,
        pub column: Number,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct KeyEvent<'a> {
        key: &'a str,
        subframe: Number,
        hoisted: Option<bool>, //TODO: Figure out what this means
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct GameOptions<'a> {
        #[serde(rename = "allow180")]
        pub allow_180: bool,
        pub allow_harddrop: bool,
        pub are: Number,
        #[serde(rename = "b2bchaining")]
        pub b2b_chaining: bool,
        #[serde(rename = "bagtype")]
        pub bag_type: &'a str, // change to enum
        #[serde(rename = "boardbuffer")]
        pub board_buffer: Number,
        #[serde(rename = "boardheight")]
        pub board_height: Number,
        #[serde(rename = "boardskin")]
        pub board_skin: &'a str,
        #[serde(rename = "boardwidth")]
        pub board_width: Number,
        pub clutch: bool,
        pub countdown: bool,
        /// how many counts before GO (for example, countdown_count = 3 means 3, 2, 1, GO, and game starts immediately on GO)
        pub countdown_count: Number,
        /// the amount of time between two counts measured in milliseconds
        pub countdown_interval: Number,
        pub display_fire: bool,
        pub display_hold: bool,
        pub display_next: bool,
        pub display_shadow: bool,
        pub display_username: bool,
        pub forfeit_time: Number,
        #[serde(rename = "fullinterval")]
        pub full_interval: Number,
        #[serde(rename = "fulloffset")]
        pub full_offset: Number,
        #[serde(rename = "g")]
        pub gravity: Number,
        #[serde(rename = "garbagecap")]
        pub garbage_cap: Number,
        #[serde(rename = "garbagecapincrease")]
        pub garbage_cap_increase: Number,
        #[serde(rename = "garbagecapmax")]
        pub garbage_cap_max: Number,
        #[serde(rename = "garbageincrease")]
        pub garbage_increase: Number,
        #[serde(rename = "garbagemargin")]
        pub garbage_margin: Number,
        #[serde(rename = "garbagemultiplier")]
        pub garbage_multiplier: Number,
        #[serde(rename = "garbagespeed")]
        pub garbage_speed: Number,
        #[serde(rename = "ghostskin")]
        pub ghost_skin: &'a str,
        #[serde(rename = "gincrease")]
        pub gravity_increase: Number,
        #[serde(rename = "gmargin")]
        pub gravity_margin: Number,
        handling: Handling,
        #[serde(rename = "hasgarbage")]
        pub has_garbage: bool,
        #[serde(rename = "infinitemovement")]
        pub infinite_movement: bool,
        pub kickset: &'a str,
        #[serde(rename = "latencypreference")]
        pub latency_preference: &'a str, //TODO: Probably has to do with passthrough, but should check
        pub lineclear_are: Number,
        #[serde(rename = "lockresets")]
        pub lock_resets: Number,
        #[serde(rename = "locktime")]
        pub lock_time: Number,
        pub manual_allowed: bool,
        #[serde(rename = "minoskin")]
        pub tetromino_skin: TetrominoSkin<'a>,
        pub mission: Option<&'a str>,
        pub mission_type: &'a str,
        #[serde(rename = "neverstopbgm")]
        pub loop_bgm: bool,
        #[serde(rename = "nextcount")]
        pub next_count: Number,
        pub objective: Value, // TODO: confirm -- this probably refers to lines cleared to finish in singleplayer games and such
        #[serde(rename = "onfail")]
        pub on_fail: Value, // TODO: Find the types of these
        #[serde(rename = "onfinish")]
        pub on_finish: Value,
        #[serde(rename = "oninteraction")]
        pub on_interaction: Value,
        pub passthrough: bool,
        pub physical: bool,
        #[serde(rename = "precountdown")]
        pub pre_countdown: Number,
        #[serde(rename = "prestart")]
        pub pre_start: Number,
        pub room_handling: bool,
        pub room_handling_arr: Number,
        pub room_handling_das: Number,
        pub room_handling_sdf: Number,
        pub seed: Number,
        pub seed_random: bool,
        pub slot_bar1: &'a str,
        pub slot_counter1: Option<&'a str>,
        pub slot_counter2: Option<&'a str>,
        pub slot_counter3: Option<&'a str>,
        pub slot_counter4: Option<&'a str>,
        pub slot_counter5: Option<&'a str>,
        #[serde(rename = "spinbonuses")]
        pub spin_bonuses: &'a str,
        pub stock: Number,
        pub username: &'a str,
        pub version: Number,
        #[serde(rename = "zoominto")]
        pub zoom_into: &'a str,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TetrominoSkin<'a> {
        i: &'a str,
        j: &'a str,
        l: &'a str,
        o: &'a str,
        s: &'a str,
        t: &'a str,
        z: &'a str,
        other: &'a str,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Killer<'a> {
        pub name: Option<&'a str>,
        #[serde(rename = "type")]
        pub kind: &'a str,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AggregateStats {
        pub apm: Number,
        pub pps: Number,
        pub vsscore: Number,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Game<'a> {
        pub bag: Vec<&'a str>,
        /// null or string (one of "ljzsoti" or "gb" for garbage)
        #[serde(borrow)]
        pub board: Vec<Vec<Option<&'a str>>>,
        pub controlling: Controlling,
        #[serde(rename = "g")]
        pub gravity: Number,
        pub handling: Handling,
        pub hold: Hold<'a>,
        pub playing: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Hold<'a> {
        pub locked: bool,
        pub piece: Option<&'a str>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Handling {
        pub arr: Number,
        pub cancel: bool,
        pub das: Number,
        pub dcd: Number,
        pub safelock: bool,
        pub sdf: Number,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Controlling {
        #[serde(rename = "lastshift")]
        pub last_shift: Number,
        pub ldas: Number,
        pub ldasiter: Number, //TODO should rename sometime
        pub lshift: bool,
        pub rdas: Number,
        pub rdasiter: Number, //TODO this too
        pub rshift: bool,
        pub softdrop: bool,
    }
}
