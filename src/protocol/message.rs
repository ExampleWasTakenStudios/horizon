use crate::{error::{DnsError, DnsResult}, protocol::{DnsHeader, DnsQuestion, DnsRecord}};

#[derive(Debug, Clone)]
pub struct DnsMessage<'a> {
    pub buf: Vec<u8>,
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion<'a>>,
    pub answers: Vec<DnsRecord<'a>>,
    pub authoritatives: Vec<DnsRecord<'a>>,
    pub additionals: Vec<DnsRecord<'a>>,
}

impl<'a> DnsMessage<'a> {
    pub fn from_bytes(bytes: Vec<u8>) -> () {
        if bytes.len() < 12 {
            //return Err(DnsError::PacketTooShort);
        }

        let header = DnsHeader::from_bytes(bytes.as_slice());
    }
}
