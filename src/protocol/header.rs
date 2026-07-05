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
    pub fn read_from<const T: usize>(buffer: &mut PacketBuffer<T>) -> Result<Self, ResponseCode> {
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
}
