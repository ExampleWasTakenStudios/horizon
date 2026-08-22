use crate::{
    error::{DnsError, DnsResult},
    protocol::DnsQuestion,
};

#[derive(Debug)]
pub struct DnsPacketBuilder {
    header: [u8; 12],
    payload: Vec<u8>,
}

impl DnsPacketBuilder {
    pub fn new() -> Self {
        Self {
            header: [0; 12],
            payload: Vec::new(),
        }
    }

    pub fn build(mut self) -> Vec<u8> {
        let mut final_packet = self.header.to_vec();
        final_packet.append(&mut self.payload);

        final_packet
    }

    pub fn id(mut self, id: u16) -> Self {
        let high = ((id & 0b1111_1111_0000_0000) >> 8) as u8;
        let low = (id & 0b0000_0000_1111_1111) as u8;

        self.header[0] = high;
        self.header[1] = low;
        self
    }

    pub fn is_response(mut self, is_response: bool) -> Self {
        self.header[2] |= (is_response as u8) << 7;
        self
    }

    pub fn op_code(mut self, op_code: u8) -> DnsResult<Self> {
        if op_code > 15 {
            return Err(DnsError::IllegalOpCodeWhileBuildingPacket);
        }

        self.header[2] |= op_code << 3;
        Ok(self)
    }

    pub fn is_authoritative(mut self, is_authoritative: bool) -> Self {
        self.header[2] |= (is_authoritative as u8) << 2;
        self
    }

    pub fn is_truncated(mut self, is_truncated: bool) -> Self {
        self.header[2] |= (is_truncated as u8) << 1;
        self
    }

    pub fn recursion_desired(mut self, recursion_desired: bool) -> Self {
        self.header[2] |= recursion_desired as u8;
        self
    }

    pub fn recursion_avail(mut self, recursion_avail: bool) -> Self {
        self.header[3] |= (recursion_avail as u8) << 7;
        self
    }

    pub fn z(mut self, z: u8) -> DnsResult<Self> {
        if z > 7 {
            return Err(DnsError::IllegalZValueWhileBuildingPacket);
        }

        self.header[3] |= z << 4;
        Ok(self)
    }

    pub fn response_code(mut self, response_code: u8) -> DnsResult<Self> {
        if response_code > 15 {
            return Err(DnsError::IllegalResponseCodeWhileBuildingPacket);
        }

        self.header[3] |= response_code;
        Ok(self)
    }

    pub fn set_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::error::DnsError;

    use super::*;

    #[test]
    fn id() {
        // ARRANGE
        let id = 1234_u16;

        // ACT
        let packet = DnsPacketBuilder::new().id(id).build();

        // ASSERT
        let high = packet[0] as u16;
        let low = packet[1] as u16;
        let result: u16 = (high << 8) | low;
        assert_eq!(result, id);
    }

    #[test]
    fn is_response() {
        // ARRANGE
        let is_response = true;

        // ACT
        let packet = DnsPacketBuilder::new().is_response(is_response).build();

        // ASSERT
        assert_eq!((packet[2] & 0b1000_0000) != 0, is_response);
    }

    #[test]
    fn valid_op_code() {
        // ARRANGE
        let op_code = 1_u8; // This OPCODE no longer exists, however since we're testing functionality of the DnsPacketBuilder here, we use it anyway.

        // ACT
        let packet = DnsPacketBuilder::new().op_code(op_code);

        // ASSERT
        assert!(packet.is_ok());
        let packet = packet.unwrap().build();
        assert_eq!(((packet[2] & 0b0111_1000) >> 3), op_code);
    }

    #[test]
    fn invalid_op_code() {
        // ARRANGE
        let op_code = 16_u8; // The maximum size for an unsigned 4-bit integer is 15.

        // ACT
        let packet = DnsPacketBuilder::new().op_code(op_code);

        // ASSERT
        assert!(packet.is_err());
        let packet = packet.unwrap_err();
        assert_eq!(packet, DnsError::IllegalOpCodeWhileBuildingPacket);
    }

    #[test]
    fn is_authoritative() {
        // ARRANGE
        let is_auth = true;

        // ACT
        let packet = DnsPacketBuilder::new().is_authoritative(is_auth).build();

        // ASSERT
        assert_eq!((packet[2] & 0b0000_0100) != 0, is_auth);
    }

    #[test]
    fn is_truncated() {
        // ARRANGE
        let is_trunc = true;

        // ACT
        let packet = DnsPacketBuilder::new().is_truncated(is_trunc).build();

        // ASSERT
        assert_eq!((packet[2] & 0b0000_0010) != 0, is_trunc);
    }

    #[test]
    fn recursion_desired() {
        // ARRANGE
        let rec_des = true;

        // ACT
        let packet = DnsPacketBuilder::new().recursion_desired(rec_des).build();

        // ASSERT
        assert_eq!((packet[2] & 0b0000_0001) != 0, rec_des);
    }

    #[test]
    fn recursion_avail() {
        // ARRANGE
        let rec_avail = true;

        // ACT
        let packet = DnsPacketBuilder::new().recursion_avail(rec_avail).build();

        // ASSERT
        assert_eq!((packet[3] & 0b1000_0000) != 0, rec_avail);
    }

    #[test]
    fn valid_z() {
        // ARRANGE
        let z = 0b010;

        // ACT
        let packet = DnsPacketBuilder::new().z(z);

        // ASSERT
        assert!(packet.is_ok());
        let packet = packet.unwrap().build();
        assert_eq!(((packet[3] & 0b0111_0000) >> 4), z);
    }

    #[test]
    fn invalid_z() {
        // ARRANGE
        let z = 8_u8;

        // ACT
        let packet = DnsPacketBuilder::new().z(z);

        // ASSERT
        assert!(packet.is_err());
        let packet = packet.unwrap_err();
        assert_eq!(packet, DnsError::IllegalZValueWhileBuildingPacket);
    }

    #[test]
    fn valid_response_code() {
        // ARRANGE
        let response_code = 1_u8;

        // ACT
        let packet = DnsPacketBuilder::new().response_code(response_code);

        // ASSERT
        assert!(packet.is_ok());
        let packet = packet.unwrap().build();
        assert_eq!((packet[3] & 0b0000_1111), response_code);
    }

    #[test]
    fn invalid_response_code() {
        // ARRANGE
        let response_code = 16_u8;

        // ACT
        let packet = DnsPacketBuilder::new().response_code(response_code);

        // ASSERT
        assert!(packet.is_err());
        let packet = packet.unwrap_err();
        assert_eq!(packet, DnsError::IllegalResponseCodeWhileBuildingPacket);
    }
}
