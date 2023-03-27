pub mod event;
mod game_type;
mod ttr;
mod ttrm;

pub use game_type::*;
pub use serde_json::Error;
pub use ttr::*;
pub use ttrm::*;

pub fn ttr_from_slice(slice: &[u8]) -> Result<Ttr, serde_json::Error> {
    serde_json::from_slice(slice)
}

pub fn ttrm_from_slice(slice: &[u8]) -> Result<Ttrm, serde_json::Error> {
    serde_json::from_slice(slice)
}
