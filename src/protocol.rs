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
    pub fn from_number(x: u16) -> Self {
        match x {
            1 => DnsClass::IN,
            255 | _ => DnsClass::WILDCARD,
        }
    }

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
    pub fn from_number(x: u16) -> Self {
        match x {
            1 => DnsType::A,
            2 => DnsType::NS,
            5 => DnsType::CNAME,
            6 => DnsType::SOA,
            12 => DnsType::PTR,
            15 => DnsType::MX,
            16 => DnsType::TXT,
            255 | _ => DnsType::WILDCARD,
        }
    }

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

pub struct DomainName {
    name: Vec<u8>,
}

impl DomainName {
    fn parse_raw(buffer: &[u8], start_position: usize) -> Result<(Vec<u8>, usize), ResponseCode> {
        let mut local_position = start_position;
        let mut jumped = false;
        let mut jump_end_position = 0;
        let mut visited_pointers = Vec::new();

        let mut out_bytes: Vec<u8> = Vec::new();

        loop {
            if local_position >= buffer.len() {
                return Err(ResponseCode::FORMERR);
            }

            let current_byte = buffer[local_position];

            // Check if current_byte is a pointer
            if (current_byte & 0xC0) == 0xC0 {
                if !jumped {
                    jump_end_position = local_position + 2;
                }

                let low_byte = buffer[local_position + 1] as u16;
                let ptr = (((current_byte as u16) ^ 0xC0) << 8 | low_byte) as usize;

                if visited_pointers.contains(&ptr) {
                    return Err(ResponseCode::FORMERR);
                }
                visited_pointers.push(ptr);

                local_position = ptr;
                jumped = true;
                continue;
            }

            // Check if current_byte is a zero terminator (0x00)
            if current_byte == 0x00 {
                if jumped {
                    local_position = jump_end_position;
                    jumped = false;
                    continue;
                }
                break;
            }

            // current_byte must be a length label
            if (current_byte as usize) >= buffer.len() {
                return Err(ResponseCode::FORMERR);
            }

            let label_start_position = local_position + 1;
            let label_end_position = local_position + (current_byte as usize);

            let labels = buffer[label_start_position..=label_end_position].to_vec();

            out_bytes.extend_from_slice(&labels);
            local_position = label_end_position;
        }

        Ok((out_bytes, local_position))
    }

    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let (bytes, bytes_consumed) = Self::parse_raw(buffer.as_slice(), buffer.get_position())?;

        buffer.advance_by(bytes_consumed);

        Ok(DomainName { name: bytes })
    }
}

pub struct AData {
    address: [u8; 4],
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

pub struct NsData {
    ns_domain_name: DomainName,
}

impl NsData {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        Ok(NsData {
            ns_domain_name: DomainName::read_from(buffer)?,
        })
    }
}

pub struct CNameData {
    c_name: DomainName,
}

impl CNameData {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        Ok(CNameData {
            c_name: DomainName::read_from(buffer)?,
        })
    }
}

pub struct SoaRecordData {
    m_name: DomainName,
    r_name: DomainName,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    minimum: u32,
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

pub struct PtrData {
    ptr_d_name: DomainName,
}

impl PtrData {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        Ok(PtrData {
            ptr_d_name: DomainName::read_from(buffer)?,
        })
    }
}

pub struct MxRecordData {
    preference: u16,
    exchange: DomainName,
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

pub struct TxtData {
    txt_data: Vec<u8>,
}

impl TxtData {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let length = buffer.read_u8()?;
        let data = buffer.read_subarray(length as usize)?;

        Ok(TxtData { txt_data: data })
    }
}

pub struct UnknownRData {
    data: Vec<u8>,
}

impl UnknownRData {
    pub fn read_from(buffer: &mut PacketBuffer, length: usize) -> Result<Self, ResponseCode> {
        Ok(UnknownRData {
            data: buffer.read_subarray(length)?,
        })
    }
}

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
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
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
    name: DomainName,
    query_type: DnsType,
    query_class: DnsClass,
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

pub struct DnsRecord {
    name: DomainName,
    r#type: DnsType,
    class: DnsClass,
    ttl: u32,
    rd_length: u16,
    r_data: DnsRecordData,
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
            DnsType::TXT => DnsRecordData::TXT(TxtData::read_from(buffer)?),
            DnsType::WILDCARD => {
                DnsRecordData::UNKNOWN(UnknownRData::read_from(buffer, rd_length as usize)?)
            }
        };

        Ok(DnsRecord {
            name,
            r#type: r#type,
            class,
            ttl,
            rd_length,
            r_data,
        })
    }
}

pub struct DnsPacket {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsRecord>,
    authoritatives: Vec<DnsRecord>,
    additionals: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn parse_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let header = DnsHeader::read_from(buffer)?;
        let mut questions: Vec<DnsQuestion> = Vec::with_capacity(header.question_count as usize);
        let mut answers: Vec<DnsRecord> = Vec::with_capacity(header.answer_count as usize);
        let mut authoritatives: Vec<DnsRecord> =
            Vec::with_capacity(header.authoritative_count as usize);
        let mut additionals: Vec<DnsRecord> = Vec::with_capacity(header.additional_count as usize);

        for _ in 0..header.question_count {
            let question = DnsQuestion::read_from(buffer)?;
            questions.push(question);
        }

        for _ in 0..header.answer_count {
            let answer = DnsRecord::read_from(buffer)?;
            answers.push(answer);
        }

        for _ in 0..header.authoritative_count {
            let authoritative = DnsRecord::read_from(buffer)?;
            authoritatives.push(authoritative);
        }

        for _ in 0..header.additional_count {
            let additional = DnsRecord::read_from(buffer)?;
            additionals.push(additional);
        }

        Ok(DnsPacket {
            header,
            questions,
            answers,
            authoritatives,
            additionals,
        })
    }
}
