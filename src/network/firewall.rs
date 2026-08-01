use crate::constants;

pub struct Firewall;

impl Firewall {
    /// Verify that a UDP query fulfills the following conditions:
    /// 1. It does not exceed the [`constants::MAX_PACKET_SIZE`]
    /// 2. It is not shorter than 12 bytes (the minimum size of a valid DNS packet)
    /// 3. First bit of the 3<sup>rd</sup> byte is not `1` (indicating the DNS packet is a query)
    pub fn verify_query(buf: &[u8], length_read: usize) -> bool {
        (12..=constants::MAX_PACKET_SIZE).contains(&length_read) && (buf[2] & 0x80) == 0
    }
}
