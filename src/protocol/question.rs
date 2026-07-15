use crate::{buffer::PacketBuffer, protocol::{DnsClass, DnsType, ResponseCode, domain::DomainName}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DnsQuestion {
    pub name: DomainName,
    pub query_type: DnsType,
    pub query_class: DnsClass,
}

impl DnsQuestion {
    pub fn read_from<const T: usize>(buffer: &mut PacketBuffer<T>) -> Result<Self, ResponseCode> {
        let q_name = DomainName::read_from(buffer)?;
        let q_type = DnsType::from_number(buffer.read_u16()?);
        let q_class = DnsClass::from_number(buffer.read_u16()?);

        Ok(DnsQuestion {
            name: q_name,
            query_type: q_type,
            query_class: q_class,
        })
    }
}
