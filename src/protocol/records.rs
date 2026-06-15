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
}
