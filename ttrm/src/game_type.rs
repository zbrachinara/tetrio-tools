use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum GameType {
    #[serde(rename = "40l")]
    FortyLine,
    League,
    Custom,
    Blitz,
}
