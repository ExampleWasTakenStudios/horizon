use crate::{
    error::{DnsError, DnsResult},
    protocol::{DnsHeader, DnsQuestion, DnsRecord},
};

#[derive(Debug, Clone)]
pub struct DnsMessage {
    pub buf: Vec<u8>,
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authoritatives: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
}

impl DnsMessage {
    pub fn from_bytes(bytes: &mut Vec<u8>) -> DnsResult<()> {
        if bytes.len() < 12 {
            return Err(DnsError::PacketTooShort);
        }

        let header = DnsHeader::from_bytes(bytes.as_slice());

        Ok(())
    }

    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> DnsResult<()> {
        Ok(())
    }
}
