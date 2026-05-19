use crate::buffer::PacketBuffer;

pub enum ResponseCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl ResponseCode {
    pub fn from_number(number: u8) -> ResponseCode {
        match number {
            1 => ResponseCode::FORMERR,
            2 => ResponseCode::SERVFAIL,
            3 => ResponseCode::NXDOMAIN,
            4 => ResponseCode::NOTIMP,
            5 => ResponseCode::REFUSED,
            0 | _ => ResponseCode::NOERROR,
        }
    }
}

pub enum DnsClass {
    IN = 1,
    WILDCARD = 255,
}

impl DnsClass {
    pub fn to_number(&self) -> u16 {
        return match *self {
            DnsClass::IN => 1,
            DnsClass::WILDCARD => 255,
        };
    }
}

pub enum DnsType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
    WILDCARD = 255,
}

impl DnsType {
    pub fn to_number(&self) -> u16 {
        return match *self {
            DnsType::A => 1,
            DnsType::NS => 2,
            DnsType::CNAME => 5,
            DnsType::SOA => 6,
            DnsType::PTR => 12,
            DnsType::MX => 15,
            DnsType::TXT => 16,
            DnsType::WILDCARD => 255,
        };
    }
}

pub struct SoaRecord {
    m_name: Vec<u8>,
    r_name: Vec<u8>,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    minimum: u32,
}

pub struct MxRecord {
    preference: u16,
    exchange: Vec<u8>,
}

pub enum DnsRecordData {
    A([u8; 4]),
    NS(Vec<u8>),
    CNAME(Vec<u8>),
    SOA(SoaRecord),
    PTR(Vec<u8>),
    MX(MxRecord),
    TXT(Vec<u8>),
}

pub struct DnsHeader {
    id: u16,

    is_response: bool,
    op_code: u8,
    is_authoritative: bool,
    is_truncated: bool,
    is_recursion_desired: bool,
    is_recursion_avail: bool,
    z: u8,
    response_code: ResponseCode,

    question_count: u16,
    answer_count: u16,
    authoritative_count: u16,
    additional_count: u16,
}

impl DnsHeader {
    pub fn from_packet_buffer(buffer: &mut PacketBuffer) -> Result<Self, &'static str> {
        let id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;

        let is_response = ((flags & 0xFFFF) >> 15) != 0; // Masks the MSB
        let op_code: u8 = ((flags & 0x87FF) >> 11) as u8;
        let is_authoritative = ((flags & 0xFBFF) >> 10) != 0;
        let is_truncated = ((flags & 0xFDFF) >> 9) != 0;
        let is_recursion_desired = ((flags & 0xFEFF) >> 8) != 0;
        let is_recursion_avail = ((flags & 0xFF7F) >> 7) != 0;
        let z = ((flags & 0xFF8F) >> 4) as u8;
        let response_code = ResponseCode::from_number((flags & 0xFFF0) as u8);

        let question_count = buffer.read_u16()?;
        let answer_count = buffer.read_u16()?;
        let authoritative_count = buffer.read_u16()?;
        let additional_count = buffer.read_u16()?;

        Ok(DnsHeader {
            id,
            is_response,
            op_code,
            is_authoritative,
            is_truncated,
            is_recursion_desired,
            is_recursion_avail,
            z,
            response_code,
            question_count,
            answer_count,
            authoritative_count,
            additional_count,
        })
    }
}

pub struct DnsQuestion {
    name: String,
    query_type: DnsType,
}

pub struct DnsRecord {
    name: Vec<u8>,
    r#type: u16,
    class: u16,
    ttl: u32,
    rd_length: u16,
    r_data: DnsRecordData,
}