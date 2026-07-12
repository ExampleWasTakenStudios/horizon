use std::io::Write;

use crate::protocol::ResponseCode;

/// A cursored buffer with length `T`.
pub struct PacketBuffer<const T: usize> {
    buffer: [u8; T],
    position: usize,
}

impl Default for PacketBuffer<1232> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const T: usize> PacketBuffer<T> {
    pub fn new() -> Self {
        PacketBuffer {
            buffer: [0; T],
            position: 0,
        }
    }

    pub fn from_raw_buffer(buffer: [u8; T]) -> Self {
        PacketBuffer {
            buffer,
            position: 0,
        }
    }

    pub fn as_slice(&self) -> &[u8; T] {
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

        if length > self.buffer.len() {
            return Err(ResponseCode::FORMERR);
        }

        for _ in 0..length {
            subarray.push(self.read_u8()?);
        }

        Ok(subarray)
    }

    /// Write a `u8` to the buffer.
    ///
    /// Returns the length written, or an error message as `&str`
    pub fn write_u8(&mut self, value: u8) -> Result<usize, &str> {
        let v = self
            .buffer
            .get_mut(self.position)
            .ok_or("Index out of bounds.")?;
        *v = value;

        self.position += 1;

        Ok(1_usize)
    }

    /// Write a `u8` to the buffer.
    ///
    /// Returns the length written, or an error message as `&str`
    pub fn write_u16(&mut self, value: u16) -> Result<usize, &str> {
        let buf = [(value & 0xFF00) as u8, (value & 0x00FF) as u8];

        let mut v = self
            .buffer
            .get_mut(self.position..=(self.position + 1))
            .ok_or("Index out of bounds.")?;
        match v.write(&buf) {
            Err(_) => {
                return Err("Could not write u16 value to PacketBuffer.");
            }
            Ok(length) => {
                self.position += length;
                return Ok(length);
            }
        };
    }

    /// Write a `u8` to the buffer.
    ///
    /// Returns the length written, or an error message as `&str`
    pub fn write_u32(&mut self, value: u32) -> Result<usize, &str> {
        let buf = [
            (value & 0xFF0000) as u8,
            (value & 0x00FF00) as u8,
            (value & 0x0000FF) as u8,
        ];

        let mut v = self
            .buffer
            .get_mut(self.position..=(self.position + 2))
            .ok_or("Index out of bounds.")?;
        match v.write(&buf) {
            Err(_) => {
                return Err("Could not write u32 value to PacketBuffer.");
            }
            Ok(length) => {
                self.position += length;
                return Ok(length);
            }
        };
    }
}
