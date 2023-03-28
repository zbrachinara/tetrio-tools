use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event<'a> {
    pub frame: u32,
    #[serde(borrow, flatten)]
    pub data: EventData<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "data")]
pub enum EventData<'a> {
    Start {},
    Full {
        #[serde(flatten, borrow)]
        data: Box<EventFull<'a>>,
    },
    Targets {}, // TODO fill in fields
    KeyDown {
        #[serde(flatten)]
        key_event: KeyEvent,
    },
    KeyUp {
        #[serde(flatten)]
        key_event: KeyEvent,
    },
    #[serde(rename = "ige")]
    InGameEvent {
        #[serde(flatten)]
        event: Box<InteractionContainer>,
    },
    End {},
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventFull<'a> {
    #[serde(rename = "aggregatestats")]
    pub aggregate_stats: AggregateStats,
    pub assumptions: Option<Value>,
    pub fire: Number,
    pub game: Game<'a>,
    #[serde(rename = "gameoverreason")]
    pub game_over_reason: Option<&'a str>,
    pub killer: Killer<'a>,
    pub options: GameOptions<'a>,
    pub replay: Value, //TODO: gonna have to see what these structs mean
    pub source: Value,
    pub stats: Value,
    pub successful: bool,
    pub targets: Vec<Value>, // TODO: probably string, but have to check
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InteractionContainer {
    pub id: Number,
    pub frame: u32,
    pub data: Interaction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Interaction {
    pub sender: String,
    pub sent_frame: Number,
    pub cid: Number,
    #[serde(flatten)]
    pub data: InteractionData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "data")]
pub enum InteractionData {
    #[serde(rename = "interaction")]
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
    pub amt: u16,
    pub x: Number,
    pub y: Number,
    pub column: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEvent {
    pub key: Key,
    pub subframe: Number,
    pub hoisted: Option<bool>, //TODO: Figure out what this means
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "hold")]
    Hold,
    #[serde(rename = "moveLeft")]
    Left,
    #[serde(rename = "moveRight")]
    Right,
    #[serde(rename = "rotateCW")]
    Clockwise,
    #[serde(rename = "rotate180")]
    Flip,
    #[serde(rename = "rotateCCW")]
    CounterClockwise,
    #[serde(rename = "softDrop")]
    SoftDrop,
    #[serde(rename = "hardDrop")]
    HardDrop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameOptions<'a> {
    #[serde(rename = "allow180")]
    pub allow_180: bool,
    pub allow_harddrop: Option<bool>,
    pub are: Option<Number>,
    #[serde(rename = "b2bchaining")]
    pub b2b_chaining: Option<bool>,
    #[serde(rename = "bagtype")]
    pub bag_type: Option<&'a str>, // change to enum
    #[serde(rename = "boardbuffer")]
    pub board_buffer: Number,
    #[serde(rename = "boardheight")]
    pub board_height: Number,
    #[serde(rename = "boardskin")]
    pub board_skin: &'a str,
    #[serde(rename = "boardwidth")]
    pub board_width: Number,
    pub clutch: Option<bool>,
    pub countdown: bool,
    /// How many counts before GO (for example, countdown_count = 3 means 3, 2, 1, GO, and game
    /// starts immediately on GO). Seems to only be present in multiplayer matches, presumably
    /// because in tetra league, the first match has five counts while the rest have three.
    pub countdown_count: Option<Number>,
    /// the amount of time between two counts measured in milliseconds
    pub countdown_interval: Number,
    pub display_fire: Option<bool>,
    pub display_hold: Option<bool>,
    pub display_next: Option<bool>,
    pub display_shadow: Option<bool>,
    pub display_username: Option<bool>,
    pub display_replay: Option<bool>,
    pub forfeit_time: Option<Number>,
    pub display_progress: Option<bool>,
    #[serde(rename = "fullinterval")]
    pub full_interval: Option<Number>,
    #[serde(rename = "fulloffset")]
    pub full_offset: Option<Number>,
    #[serde(rename = "g")]
    pub gravity: Option<f32>,
    #[serde(rename = "garbagecap")]
    pub garbage_cap: u16,
    #[serde(rename = "garbagecapincrease")]
    pub garbage_cap_increase: Option<Number>,
    #[serde(rename = "garbagecapmax")]
    pub garbage_cap_max: Option<Number>,
    #[serde(rename = "garbageincrease")]
    pub garbage_increase: Option<Number>,
    #[serde(rename = "garbagemargin")]
    pub garbage_margin: Option<Number>,
    #[serde(rename = "garbagemultiplier")]
    pub garbage_multiplier: Option<Number>,
    /// After garbage is acknowledged by a client, there is a delay before it takes effect. After
    /// garbage takes effect, it will be applied to the board on the next hard drop. This quantity
    /// describes the delay in number of frames (not subframes)
    #[serde(rename = "garbagespeed")]
    pub garbage_speed: u32,
    #[serde(rename = "ghostskin")]
    pub ghost_skin: &'a str,
    #[serde(rename = "gbase")]
    pub gravity_base: Option<f32>,
    #[serde(rename = "gspeed")]
    pub gravity_speed: Option<Number>,
    #[serde(rename = "gincrease")]
    pub gravity_increase: Option<f32>,
    #[serde(rename = "gmargin")]
    pub gravity_margin: Option<Number>,
    pub handling: Option<Handling>,
    #[serde(rename = "hasgarbage")]
    pub has_garbage: Option<bool>,
    #[serde(rename = "infinitemovement")]
    pub infinite_movement: Option<bool>,
    pub kickset: &'a str,
    #[serde(rename = "latencypreference")]
    pub latency_preference: Option<&'a str>, //TODO: Probably has to do with passthrough, but should check
    pub lineclear_are: Option<Number>,
    #[serde(rename = "lockresets")]
    pub lock_resets: Option<Number>, // TODO should default to 16, but could depend on the gamemode
    #[serde(rename = "locktime")]
    pub lock_time: Option<u64>, // TODO should default to 30, but could depend on the gamemode
    pub manual_allowed: Option<bool>,
    #[serde(rename = "minoskin")]
    pub tetromino_skin: TetrominoSkin<'a>,
    pub mission: Option<&'a str>,
    pub mission_type: Option<&'a str>,
    #[serde(rename = "neverstopbgm")]
    pub loop_bgm: Option<bool>,
    #[serde(rename = "bgmnoreset")]
    pub bgm_no_reset: Option<bool>, // TODO compare with neverstopbgm (could be the same or opposite thing)
    #[serde(rename = "nextcount")]
    pub next_count: Number,
    pub objective: Value, // TODO: confirm -- this probably refers to lines cleared to finish in singleplayer games and such
    #[serde(rename = "onfail")]
    pub on_fail: Option<Value>, // TODO: Find the types of these
    #[serde(rename = "onfinish")]
    pub on_finish: Option<Value>,
    #[serde(rename = "oninteraction")]
    pub on_interaction: Option<Value>,
    pub passthrough: Option<bool>,
    pub physical: bool,
    #[serde(rename = "precountdown")]
    pub pre_countdown: Number,
    #[serde(rename = "prestart")]
    pub pre_start: Number,
    // TODO check if room handling can be grouped together
    pub room_handling: Option<bool>,
    pub room_handling_arr: Option<Number>,
    pub room_handling_das: Option<Number>,
    pub room_handling_sdf: Option<Number>,
    pub seed: u64,
    pub seed_random: bool,
    pub slot_bar1: Option<&'a str>,
    pub slot_bar2: Option<&'a str>,
    pub slot_counter1: Option<&'a str>,
    pub slot_counter2: Option<&'a str>,
    pub slot_counter3: Option<&'a str>,
    pub slot_counter4: Option<&'a str>,
    pub slot_counter5: Option<&'a str>,
    #[serde(rename = "spinbonuses")]
    pub spin_bonuses: Option<&'a str>,
    pub stock: Option<Number>,
    pub username: Option<&'a str>,
    pub version: Number, // TODO Use this!!!
    #[serde(rename = "zoominto")]
    pub zoom_into: &'a str,
    #[serde(rename = "anchorseed")]
    pub anchor_seed: Option<bool>,
    pub can_retry: Option<bool>,
    /// Option on forty-line games (maybe blitz as well) to indicate whether or not player used pro
    /// mode (turns on indicators for time left in blitz and lines left to clear in forty-line)
    #[serde(rename = "pro")]
    pub pro_mode: Option<bool>,
    /// Option on forty-line and blitz games to indicate if the player has stride mode turned on.
    /// This option allows the player to reset the game with less effort, having the effect of a
    /// quicker countdown to open the game
    pub stride: Option<bool>,
    pub no_szo: Option<bool>,
    #[serde(rename = "combotable")]
    pub combo_table: Option<&'a str>,
    #[serde(rename = "garbageblocking")]
    pub garbage_blocking: Option<&'a str>,
    pub levels: Option<bool>,
    #[serde(rename = "masterlevels")]
    pub master_levels: Option<bool>,
    #[serde(rename = "startinglevel")]
    pub starting_level: Option<Number>,
    #[serde(rename = "levelspeed")]
    pub level_speed: Option<f32>,
    #[serde(rename = "levelstatic")]
    pub level_static: Option<bool>,
    #[serde(rename = "levelstaticspeed")]
    pub level_static_speed: Option<Number>,
    #[serde(rename = "x_resulttype")]
    pub custom_metric: Option<&'a str>,
    #[serde(rename = "objective_type")]
    pub custom_objective: Option<&'a str>,
    pub objective_count: Option<Number>,
    pub objective_time: Option<Number>,
    #[serde(rename = "topoutisclear")]
    pub topout_clear: Option<bool>,
    pub absolute_lines: Option<bool>,
    pub song: Option<&'a str>,
    pub pro_alert: Option<bool>,
    pub pro_retry: Option<bool>,
    #[serde(rename = "nolockout")]
    pub no_lockout: Option<bool>,
    #[serde(rename = "survivalmode")]
    pub survival_mode: Option<&'a str>,
    pub survival_messiness: Option<u64>,
    pub survival_cap: Option<u64>,
    pub survival_layer_amt: Option<u64>,
    pub survival_layer_non: Option<bool>,
    pub survival_layer_min: Option<u64>,
    pub survival_timer_itv: Option<u64>,
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

/// Describes timings which relate to the effect of player controls. All floating-point values
/// have a precision of 0.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handling {
    /// From tetrio: "Automatic Repeat Rate: the speed at which tetrominoes move when holding
    /// down movement keys, measured in frames per movement."
    pub arr: f64,
    /// From tetrio: "If enabled, DAS charge is cancelled when you change directions."
    pub cancel: bool,
    /// From tetrio: "Delayed Auto Shift: the time between the initial keypress and the start of
    /// its automatic repeat movement, measured in frames."
    pub das: f64,
    /// From tetrio: "DAS Cut Delay: if not 0, any ongoing DAS movement will pause for a set
    /// amount of time after dropping/rotating a piece, measured in frames."
    pub dcd: f64,
    /// From tetrio: "If enabled, when a piece locks on its own, the hard drop key becomes
    /// unavailable for a few frame. This prevents accidental hard drops."
    pub safelock: bool,
    /// From tetrio: "Soft Drop Factor: the factor with which soft drops change the gravity
    /// speed."
    pub sdf: f64,
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
