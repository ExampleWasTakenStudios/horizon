use tokio::io::AsyncWriteExt;

use crate::{buffer::PacketBuffer, protocol::{DnsClass, DnsType, DomainName, ResponseCode}};

#[derive(Debug)]
pub struct DnsQuestion {
    pub name: DomainName,
    pub query_type: DnsType,
    pub query_class: DnsClass,
}

impl DnsQuestion {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let q_name = DomainName::read_from(buffer)?;
        let q_type = DnsType::from_number(buffer.read_u16()?);
        let q_class = DnsClass::from_number(buffer.read_u16()?);

        Ok(DnsQuestion {
            name: q_name,
            query_type: q_type,
            query_class: q_class,
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.write_all(&self.name.to_vec());
        buffer.write_u16(self.query_type.to_number());
        buffer.write_u16(self.query_class.to_number());

        buffer
    }
}
