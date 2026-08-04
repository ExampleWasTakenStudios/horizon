use crate::constants;

pub struct Firewall;

impl Firewall {
    /// Verify that a UDP query fulfills the following conditions:
    /// 1. It does not exceed the [`constants::MAX_PACKET_SIZE`]
    /// 2. It is not shorter than 12 bytes (the minimum size of a valid DNS packet)
    /// 3. First bit of the 3<sup>rd</sup> byte is not `1` (indicating the DNS packet is a query)
    pub fn verify_query(buf: &[u8]) -> bool {
        (12..=constants::MAX_PACKET_SIZE).contains(&buf.len()) && (buf[2] & 0x80) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_query_is_accepted() {
        let buf = [0_u8; 12];

        let is_valid = Firewall::verify_query(&buf);

        assert!(is_valid);
    }

    #[test]
    fn test_empty_query_is_rejected() {
        let buf = [0_u8; 0];

        let is_valid = Firewall::verify_query(&buf);

        assert!(!is_valid);
    }

    #[test]
    fn test_too_short_query_is_rejected() {
        let buf = [0_u8; 5];

        let is_valid = Firewall::verify_query(&buf);

        assert!(!is_valid);
    }

    #[test]
    fn test_too_long_query_is_rejected() {
        let buf = [0_u8; constants::MAX_PACKET_SIZE + 1];

        let is_valid = Firewall::verify_query(&buf);

        assert!(!is_valid);
    }

    #[test]
    fn test_response_is_rejected() {
        let buf = [0xFF_u8; 12];

        for i in &buf {
            println!("i: {i}");
        }

        let is_valid = Firewall::verify_query(&buf);

        assert!(!is_valid);
    }
}
