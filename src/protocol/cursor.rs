use crate::error::{DnsError, DnsResult};

#[derive(Debug)]
pub struct DnsCursor<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> DnsCursor<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn read_u8(&mut self) -> DnsResult<u8> {
        match self.buf.get(self.pos) {
            Some(v) => {
                self.pos += 1;
                Ok(*v)
            }
            None => Err(DnsError::CursorOutOfBounds),
        }
    }

    pub fn read_u16(&mut self) -> DnsResult<u16> {
        match self.buf.get(self.pos..self.pos + 2) {
            Some(v) => {
                self.pos += 2;
                Ok(u16::from_be_bytes([v[0], v[1]]))
            }
            None => Err(DnsError::CursorOutOfBounds),
        }
    }

    pub fn read_u32(&mut self) -> DnsResult<u32> {
        match self.buf.get(self.pos..self.pos + 4) {
            Some(v) => {
                self.pos += 4;
                Ok(u32::from_be_bytes([v[0], v[1], v[2], v[3]]))
            }
            None => Err(DnsError::CursorOutOfBounds),
        }
    }

    pub fn read_slice(&mut self, length: usize) -> DnsResult<&[u8]> {
        match self.buf.get(self.pos..self.pos + length) {
            Some(v) => {
                self.pos += length;
                Ok(v)
            }
            None => Err(DnsError::CursorOutOfBounds),
        }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Clones the buffer that this cursor wraps. It ***does not*** clone the Cursor.
    pub fn clone_buf(&self) -> &'a [u8] {
        self.buf.clone()
    }
}
