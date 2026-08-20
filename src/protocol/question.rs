use crate::protocol::{DnsQClass, DnsQType, DomainName};

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub q_name: DomainName,
    pub q_type: DnsQType,
    pub q_class: DnsQClass,
}
