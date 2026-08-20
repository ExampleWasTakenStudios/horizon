use crate::protocol::{DnsClass, DnsType, DomainName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsRecord {
    pub name: DomainName,
    pub r#type: DnsType,
    pub class: DnsClass,
    pub ttl: u32,
    pub rd_length: u16,
    pub r_data: Vec<u8>,
}
