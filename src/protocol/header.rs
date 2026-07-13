use crate::{buffer::PacketBuffer, protocol::enums::ResponseCode};

#[derive(Debug, Clone)]
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

    pub fn to_bytes<const T: usize>(&self, buffer: &mut PacketBuffer<T>) -> Result<usize, String> {
        if buffer.get_position() != 0 {
            return Err(
                "PacketBuffer position must be 0 at the beginning of header translation"
                    .to_string(),
            );
        }

        let mut length_written = buffer.write_u16(self.id)?;

        length_written += buffer.write_u16(self.flags_to_bitfield())?;

        length_written += buffer.write_u16(self.question_count)?;
        length_written += buffer.write_u16(self.answer_count)?;
        length_written += buffer.write_u16(self.authoritative_count)?;
        length_written += buffer.write_u16(self.additional_count)?;

        Ok(length_written)
    }

    fn flags_to_bitfield(&self) -> u16 {
        let is_response = (self.is_response as u16) << 15;
        let op_code = ((self.op_code as u16) & 0x0F) << 11;
        let is_authoritative = (self.is_authoritative as u16) << 10;
        let is_truncated = (self.is_truncated as u16) << 9;
        let is_recursion_desired = (self.is_recursion_desired as u16) << 8;
        let is_recursion_avail = (self.is_recursion_avail as u16) << 7;
        let z = ((self.z as u16) & 0x07) << 4;
        let response_code = (self.response_code as u16) & 0x0F;

        (is_response
            | op_code
            | is_authoritative
            | is_truncated
            | is_recursion_desired
            | is_recursion_avail
            | z
            | response_code).to_be()
    }
}
