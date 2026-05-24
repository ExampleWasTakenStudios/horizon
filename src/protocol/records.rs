use tokio::io::AsyncWriteExt;

use crate::{
    buffer::PacketBuffer,
    protocol::{
        CharString,
        domain::DomainName,
        enums::{DnsClass, DnsType, ResponseCode},
    },
};

#[derive(Debug)]
pub struct AData {
    pub address: [u8; 4],
}

impl AData {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        Ok(AData {
            address: [
                buffer.read_u8()?,
                buffer.read_u8()?,
                buffer.read_u8()?,
                buffer.read_u8()?,
            ],
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let buffer: Vec<u8> = self.address.to_vec();
        buffer
    }
}

#[derive(Debug)]
pub struct NsData {
    pub ns_domain_name: DomainName,
}

impl NsData {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        Ok(NsData {
            ns_domain_name: DomainName::read_from(buffer)?,
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.ns_domain_name.to_vec()
    }
}

#[derive(Debug)]
pub struct CNameData {
    pub c_name: DomainName,
}

impl CNameData {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        Ok(CNameData {
            c_name: DomainName::read_from(buffer)?,
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.c_name.to_vec()
    }
}

#[derive(Debug)]
pub struct SoaRecordData {
    pub m_name: DomainName,
    pub r_name: DomainName,
    pub serial: u32,
    pub refresh: u32,
    pub retry: u32,
    pub expire: u32,
    pub minimum: u32,
}

impl SoaRecordData {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let m_name = DomainName::read_from(buffer)?;
        let r_name = DomainName::read_from(buffer)?;
        let serial = buffer.read_u32()?;
        let refresh = buffer.read_u32()?;
        let retry = buffer.read_u32()?;
        let expire = buffer.read_u32()?;
        let minimum = buffer.read_u32()?;

        Ok(SoaRecordData {
            m_name,
            r_name,
            serial,
            refresh,
            retry,
            expire,
            minimum,
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        buffer.write_all(&self.m_name.to_vec());
        buffer
    }
}

#[derive(Debug)]
pub struct PtrData {
    pub ptr_d_name: DomainName,
}

impl PtrData {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        Ok(PtrData {
            ptr_d_name: DomainName::read_from(buffer)?,
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        buffer.write_all(&self.ptr_d_name.to_vec());
        buffer
    }
}

#[derive(Debug)]
pub struct MxRecordData {
    pub preference: u16,
    pub exchange: DomainName,
}

impl MxRecordData {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let preference = buffer.read_u16()?;
        let exchange = DomainName::read_from(buffer)?;

        Ok(MxRecordData {
            preference,
            exchange,
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        buffer.write_u16(self.preference);
        buffer.write_all(&self.exchange.to_vec());
        buffer
    }
}

#[derive(Debug)]
pub struct TxtData {
    pub txt_data: CharString,
}

impl TxtData {
    pub fn read_from(buffer: &mut PacketBuffer, rd_length: u16) -> Result<Self, ResponseCode> {
        Ok(TxtData {
            txt_data: CharString::read_from(buffer, rd_length)?,
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.txt_data.to_vec().clone().to_vec()
    }
}

#[derive(Debug)]
pub struct UnknownRData(Vec<u8>);

impl UnknownRData {
    pub fn read_from(buffer: &mut PacketBuffer, rd_length: usize) -> Result<Self, ResponseCode> {
        Ok(UnknownRData(buffer.read_subarray(rd_length)?))
    }
}

#[derive(Debug)]
pub enum DnsRecordData {
    A(AData),
    NS(NsData),
    CNAME(CNameData),
    SOA(SoaRecordData),
    PTR(PtrData),
    MX(MxRecordData),
    TXT(TxtData),
    UNKNOWN(UnknownRData),
}

#[derive(Debug)]
pub struct DnsRecord {
    pub name: DomainName,
    pub r#type: DnsType,
    pub class: DnsClass,
    pub ttl: u32,
    pub rd_length: u16,
    pub r_data: DnsRecordData,
}

impl DnsRecord {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let name = DomainName::read_from(buffer)?;
        let r#type = DnsType::from_number(buffer.read_u16()?);
        let class = DnsClass::from_number(buffer.read_u16()?);
        let ttl = buffer.read_u32()?;
        let rd_length = buffer.read_u16()?;
        let r_data: DnsRecordData = match r#type {
            DnsType::A => DnsRecordData::A(AData::read_from(buffer)?),
            DnsType::NS => DnsRecordData::NS(NsData::read_from(buffer)?),
            DnsType::CNAME => DnsRecordData::CNAME(CNameData::read_from(buffer)?),
            DnsType::MX => DnsRecordData::MX(MxRecordData::read_from(buffer)?),
            DnsType::PTR => DnsRecordData::PTR(PtrData::read_from(buffer)?),
            DnsType::SOA => DnsRecordData::SOA(SoaRecordData::read_from(buffer)?),
            DnsType::TXT => DnsRecordData::TXT(TxtData::read_from(buffer, rd_length)?),
            DnsType::WILDCARD => {
                DnsRecordData::UNKNOWN(UnknownRData::read_from(buffer, rd_length as usize)?)
            }
        };

        Ok(DnsRecord {
            name,
            r#type,
            class,
            ttl,
            rd_length,
            r_data,
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        match self.r#type {
            
        }
    }
}
