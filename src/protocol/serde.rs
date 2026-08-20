use crate::constants;

mod deserialize;

pub type RawMessage = [u8; constants::MAX_PACKET_SIZE];
