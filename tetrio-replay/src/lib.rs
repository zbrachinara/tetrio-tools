#![feature(drain_filter)]

mod board;
mod reconstruct;
mod rng;

pub use reconstruct::reconstruct;
pub use ttrm;
pub use viewtris; // re-export of ttrm crate
