use crate::protocol::ResponseCode;

pub struct PacketBuffer {
    buffer: [u8; 512],
    position: usize,
}

impl Default for PacketBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a cursored buffer with length 512.
/// It features special methods to read the buffer that keep track of the cursor i.e. the current location in the buffer.
impl PacketBuffer {
    pub fn new() -> Self {
        PacketBuffer {
            buffer: [0; 512],
            position: 0,
        }
    }

    pub fn from_raw_buffer(buffer: [u8; 512]) -> Self {
        PacketBuffer {
            buffer,
            position: 0,
        }
    }

    pub fn as_slice(&self) -> &[u8; 512] {
        &self.buffer
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn advance_by(&mut self, amount: usize) {
        self.position += amount;
    }

    pub fn read_u8(&mut self) -> Result<u8, ResponseCode> {
        if self.position >= 512 {
            return Err(ResponseCode::FORMERR);
        }

        let value = self.buffer[self.position];
        self.position += 1;

        Ok(value)
    }

    pub fn read_u16(&mut self) -> Result<u16, ResponseCode> {
        let value = ((self.read_u8()? as u16) << 8) | (self.read_u8()? as u16);

        Ok(value)
    }

    pub fn read_u32(&mut self) -> Result<u32, ResponseCode> {
        let value = ((self.read_u8()? as u32) << 24)
            | ((self.read_u8()? as u32) << 16)
            | ((self.read_u8()? as u32) << 8)
            | (self.read_u8()? as u32);

        Ok(value)
    }

    pub fn read_subarray(&mut self, length: usize) -> Result<Vec<u8>, ResponseCode> {
        let mut subarray: Vec<u8> = Vec::new();
        
        for _ in 0..length  {
            subarray.push(self.read_u8()?);
        }

        Ok(subarray)
    }
}
