use tokio::io::AsyncWriteExt;

use crate::{buffer::PacketBuffer, protocol::enums::ResponseCode};

#[derive(Debug)]
pub struct DnsHeader {
    pub id: u16,

    pub is_response: bool,
    pub op_code: u8,
    pub is_authoritative: bool,
    pub is_truncated: bool,
    pub is_recursion_desired: bool,
    pub is_recursion_avail: bool,
    pub z: u8,
    pub response_code: ResponseCode,

    pub question_count: u16,
    pub answer_count: u16,
    pub authoritative_count: u16,
    pub additional_count: u16,
}

impl DnsHeader {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;

        let is_response = (flags >> 15) & 1 != 0;
        let op_code = ((flags >> 11) & 0x0F) as u8;
        let is_authoritative = (flags >> 10) & 1 != 0;
        let is_truncated = (flags >> 9) & 1 != 0;
        let is_recursion_desired = (flags >> 8) & 1 != 0;
        let is_recursion_avail = (flags >> 7) & 1 != 0;
        let z = ((flags >> 4) & 0x07) as u8;
        let response_code = ResponseCode::from_number((flags & 0x0F) as u8);

        let question_count = buffer.read_u16()?;
        let answer_count = buffer.read_u16()?;
        let authoritative_count = buffer.read_u16()?;
        let additional_count = buffer.read_u16()?;

        Ok(DnsHeader {
            id,
            is_response,
            op_code,
            is_authoritative,
            is_truncated,
            is_recursion_desired,
            is_recursion_avail,
            z,
            response_code,
            question_count,
            answer_count,
            authoritative_count,
            additional_count,
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(12);

        // --- ID ---
        buffer.write_u16(self.id);

        // --- Flags ---
        let mut flags: u16 = 0;
        flags |= (self.is_response as u16) << 15;
        flags |= (self.op_code as u16 & 0b1111) << 11;
        flags |= (self.is_authoritative as u16) << 10;
        flags |= (self.is_truncated as u16) << 9;
        flags |= (self.is_recursion_desired as u16) << 8;
        flags |= (self.is_recursion_avail as u16) << 7;
        flags |= (self.z as u16 & 0b111) << 6;
        flags |= self.response_code as u16 & 0b1111;

        buffer.write_u16(flags);

        // --- Question Count ---
        buffer.write_u16(self.question_count);

        // --- Answers Count ---
        buffer.write_u16(self.answer_count);

        // --- Authoritative Count ---
        buffer.write_u16(self.authoritative_count);

        // --- Additionals Count ---
        buffer.write_u16(self.additional_count);

        buffer
    }
}
