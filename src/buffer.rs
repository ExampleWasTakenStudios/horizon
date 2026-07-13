use std::io::Write;

use crate::protocol::ResponseCode;

/// A cursored buffer with length `T`.
pub struct PacketBuffer<const T: usize> {
    buffer: [u8; T],
    position: usize,
}

impl<const T: usize> Default for PacketBuffer<T> {
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

    pub fn to_slice(&self) -> &[u8; T] {
        &self.buffer
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn advance_by(&mut self, amount: usize) {
        self.position += amount;
    }

    pub fn read_u8(&mut self) -> Result<u8, ResponseCode> {
        if self.position >= T {
            return Err(ResponseCode::FORMERR);
        }

        let value = match self.buffer.get(self.position) {
            None => {
                return Err(ResponseCode::FORMERR);
            }
            Some(v) => *v,
        };
        self.position += 1;

        Ok(value)
    }

    pub fn read_u16(&mut self) -> Result<u16, ResponseCode> {
        let value = ((self.read_u8()? as u16) << 8) | (self.read_u8()? as u16) & 0x00FF;

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

        if self.position + length > self.buffer.len() {
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
        let buf = [((value & 0xFF00) >> 8) as u8, (value & 0x00FF) as u8];

        let mut v = self
            .buffer
            .get_mut(self.position..=(self.position + 1))
            .ok_or("Index out of bounds.")?;
        match v.write(&buf) {
            Err(_) => Err("Could not write u16 value to PacketBuffer."),
            Ok(length) => {
                self.position += length;
                Ok(length)
            }
        }
    }

    /// Write a `u8` to the buffer.
    ///
    /// Returns the length written, or an error message as `&str`
    pub fn write_u32(&mut self, value: u32) -> Result<usize, &str> {
        let buf = [
            ((value & 0xFF_00_00_00) >> 24) as u8,
            ((value & 0x00_FF_00_00) >> 16) as u8,
            ((value & 0x00_00_FF_00) >> 8) as u8,
            (value & 0x00_00_00_FF) as u8,
        ];

        let mut v = self
            .buffer
            .get_mut(self.position..=(self.position + 2))
            .ok_or("Index out of bounds.")?;
        match v.write(&buf) {
            Err(_) => Err("Could not write u32 value to PacketBuffer."),
            Ok(length) => {
                self.position += length;
                Ok(length)
            }
        }
    }

    /// Write a &[u8] to the buffer.
    ///
    /// Returns the length written, or an error message as `&str`.
    pub fn write_subarray(&mut self, array: &[u8]) -> Result<usize, &str> {
        let mut v = self
            .buffer
            .get_mut(self.position..(self.position + array.len()))
            .ok_or("Index out of bounds.")?;
        match v.write(array) {
            Err(_) => Err("Could not write array to PacketBuffer."),
            Ok(length) => {
                self.position += length;
                Ok(length)
            }
        }
    }
}
