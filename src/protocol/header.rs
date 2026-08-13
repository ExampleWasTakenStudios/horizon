use crate::error::DnsResult;

#[derive(Debug, Clone)]
pub struct DnsHeader {
    pub id: u16,
    pub is_response: bool,
    pub op_code: u8,
    pub is_authoritative: bool,
    pub is_truncated: bool,
    pub recursion_desired: bool,
    pub recursion_avail: bool,
    pub z: u8,
    pub response_code: u8,
    pub questions: u16,
    pub answers: u16,
    pub authoritatives: u16,
    pub additionals: u16,
}

impl DnsHeader {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let id = u16::from_be_bytes([bytes[0], bytes[1]]);

        let flags = u16::from_be_bytes([bytes[2], bytes[3]]);
        let is_response: bool = (flags >> 15) != 0;
        let op_code: u8 = ((flags >> 11) & 0xF) as u8;
        let is_authoritative: bool = ((flags >> 10) & 0x1) != 0;
        let is_truncated: bool = ((flags >> 9) & 0x1) != 0;
        let recursion_desired: bool = (flags >> 8 & 0x1) != 0;
        let recursion_avail: bool = ((flags >> 7) & 0x1) != 0;
        let z: u8 = ((flags >> 4) & 0x5) as u8;
        let response_code: u8 = (flags & 0xF) as u8;

        let questions = u16::from_be_bytes([bytes[4], bytes[5]]);
        let answers = u16::from_be_bytes([bytes[6], bytes[7]]);
        let authoritatives = u16::from_be_bytes([bytes[8], bytes[9]]);
        let additionals = u16::from_be_bytes([bytes[10], bytes[11]]);

        Self {
            id,
            is_response,
            op_code,
            is_authoritative,
            is_truncated,
            recursion_desired,
            recursion_avail,
            z,
            response_code,
            questions,
            answers,
            authoritatives,
            additionals,
        }
    }

    pub fn to_bytes(&self, buf: &mut Vec<u8>) {
        // Push ID
        buf.extend(self.id.to_be_bytes());

        // Build first 8 bits of the flags field
        let mut flags: u8 = 0;
        flags |= (self.is_response as u8) << 7;
        flags |= (self.op_code) << 3;
        flags |= (self.is_authoritative as u8) << 2;
        flags |= (self.is_truncated as u8) << 1;
        flags |= self.recursion_desired as u8;
        buf.push(flags);

        flags = 0;
        flags |= (self.recursion_avail as u8) << 7;
        flags |= self.z << 4;
        flags |= self.response_code;
        buf.push(flags);

        // Push question count
        buf.extend(self.questions.to_be_bytes());

        // Push answer count
        buf.extend(self.authoritatives.to_be_bytes());

        // Push authoritative count
        buf.extend(self.authoritatives.to_be_bytes());

        // Push additional count
        buf.extend(self.additionals.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::unusual_byte_groupings)]
    #[rustfmt::skip]
    const VALID_QUERY_BYTES: [u8; 12] = [
        0b0000_0101, 0b0010_0001, // ID
        0b0_0000_0_0_1, 0b0_000_0000, // QR_OPCODE_AA_TC_RD - RA_Z_RCODE
        0b0000_0000, 0b0000_0001, // QDCOUNT
        0b0000_0000, 0b0000_0000, // ANCOUNT
        0b0000_0000, 0b0000_0000, // NSCOUNT
        0b0000_0000, 0b0000_0000, // ARCOUNT
    ];

    const VALID_QUERY_HEADER: DnsHeader = DnsHeader {
        id: 1313,
        is_response: false,
        op_code: 0,
        is_authoritative: false,
        is_truncated: false,
        recursion_desired: true,
        recursion_avail: false,
        z: 0,
        response_code: 0,
        questions: 1,
        answers: 0,
        authoritatives: 0,
        additionals: 0,
    };

    #[test]
    fn test_deserialize_from_valid_bytes() {
        let header = DnsHeader::from_bytes(&VALID_QUERY_BYTES);

        assert_eq!(header.id, 1313);
        assert!(!header.is_response);
        assert_eq!(header.op_code, 0);
        assert!(!header.is_authoritative);
        assert!(!header.is_truncated);
        assert!(header.recursion_desired);
        assert!(!header.recursion_avail);
        assert_eq!(header.z, 0);
        assert_eq!(header.response_code, 0);
        assert_eq!(header.questions, 1);
        assert_eq!(header.answers, 0);
        assert_eq!(header.authoritatives, 0);
        assert_eq!(header.additionals, 0);
    }

    #[test]
    fn test_serialize_from_valid_object() {
        let mut buf = Vec::<u8>::with_capacity(12);
        VALID_QUERY_HEADER.to_bytes(&mut buf);

        assert_eq!(buf, VALID_QUERY_BYTES);
    }
}
