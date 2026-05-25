use crate::{buffer::PacketBuffer, protocol::ResponseCode};

#[derive(Debug)]
pub struct CharString {
    buffer: Vec<u8>,
}

impl CharString {
    pub fn read_from(buffer: &mut PacketBuffer, length: u16) -> Result<Self, ResponseCode> {
        let mut string = Vec::with_capacity(length as usize);
        let mut bytes_read: u16 = 0;

        while bytes_read < length {
            let length = buffer.read_u8()?;
            let mut data = buffer.read_subarray(length as usize)?;

            string.append(&mut data);

            // Add 1 for the length byte itself, plus the length of the string
            bytes_read += 1 + length as u16;
        }

        Ok(CharString { buffer: string })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.buffer.clone()
    }
}
