use crate::constants;

mod deserialize;
pub use deserialize::*;

pub type RawMessage = [u8; constants::MAX_DGRAM_SIZE];
