use crate::buffer::PacketBuffer;

#[derive(Debug)]
pub enum ResponseCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl ResponseCode {
    #[allow(clippy::wildcard_in_or_patterns)] // Added for readability
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

#[derive(Debug)]
pub enum DnsClass {
    IN = 1,
    WILDCARD = 255,
}

impl DnsClass {
    #[allow(clippy::wildcard_in_or_patterns)] // Added for readability
    pub fn from_number(x: u16) -> Self {
        match x {
            1 => DnsClass::IN,
            255 | _ => DnsClass::WILDCARD,
        }
    }

    pub fn to_number(&self) -> u16 {
        match *self {
            DnsClass::IN => 1,
            DnsClass::WILDCARD => 255,
        }
    }
}

#[derive(Debug)]
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
    #[allow(clippy::wildcard_in_or_patterns)] // Added for readability
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
        match *self {
            DnsType::A => 1,
            DnsType::NS => 2,
            DnsType::CNAME => 5,
            DnsType::SOA => 6,
            DnsType::PTR => 12,
            DnsType::MX => 15,
            DnsType::TXT => 16,
            DnsType::WILDCARD => 255,
        }
    }
}

#[derive(Debug)]
pub struct DomainName {
    /// A `Vec` containing all labels that make up the domain name.
    ///
    /// A label is stored as a `Vec` holding the individual bits.
    /// This results in the nested type structure `Vec<Vec<u8>>`.
    pub labels: Vec<Vec<u8>>,
}

impl DomainName {
    fn parse_raw(
        buffer: &[u8],
        start_position: usize,
    ) -> Result<(Vec<Vec<u8>>, usize), ResponseCode> {
        // The local cursor for parsing `buffer`.
        let mut local_position = start_position;
        // Indicates that a jump is active
        let mut jumped = false;
        // Holds the value to which `local_position` should return to, once the currently active jump is complete
        let mut return_position: usize = 0;

        // Holds all parsed labels - this will be returned
        let mut labels: Vec<Vec<u8>> = Vec::new();

        loop {
            let current_byte = *buffer.get(local_position).ok_or(ResponseCode::FORMERR)?;

            // Check if current_byte is a pointer
            if (current_byte & 0xC0) == 0xC0 {
                // Decode the pointer
                let low_byte = *buffer
                    .get(local_position + 1)
                    .ok_or(ResponseCode::FORMERR)? as u16;
                let ptr = ((((current_byte as u16) ^ 0xC0) << 8) | low_byte) as usize;

                // Check that pointer location is less than current position as per the RFC1035
                // This also mathematically prevents pointer loops as a new pointer must always be less than the current position.
                if ptr >= local_position {
                    return Err(ResponseCode::FORMERR);
                }

                // Check if a jump is currently active
                if jumped {
                    local_position = ptr;
                    continue;
                }

                return_position = local_position + 2;
                local_position = ptr;
                jumped = true;
                continue;
            }

            // current_byte is not a pointer so we check if it is a zero terminator (0x00)
            if current_byte == 0x00 {
                if jumped {
                    // If we jumped to local_position, we need to return to wherever we set return_position before we jumped.
                    local_position = return_position;
                } else {
                    // Since we didn't jump, we advance the local_position by one to move past the current position which is the zero terminator
                    local_position += 1;
                }
                break;
            }

            // current_byte is neither a pointer nor a zero terminator - therefore it must be a length byte indicating the length of the following label
            let length = current_byte as usize;
            local_position += 1;

            let label = buffer
                .get(local_position..local_position + length)
                .ok_or(ResponseCode::FORMERR)?
                .to_vec();
            labels.push(label);

            local_position += length;
        }

        Ok((labels, local_position - start_position))
    }

    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let (labels, bytes_consumed) = Self::parse_raw(buffer.as_slice(), buffer.get_position())?;

        buffer.advance_by(bytes_consumed);

        Ok(DomainName { labels })
    }
}

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
    pub txt_data: Vec<u8>,
}

impl TxtData {
    pub fn read_from(buffer: &mut PacketBuffer, rd_length: u16) -> Result<Self, ResponseCode> {
        let mut txt_data = Vec::with_capacity(rd_length as usize);
        let mut bytes_read: u16 = 0;

        while bytes_read < rd_length {
            let length = buffer.read_u8()?;
            let mut data = buffer.read_subarray(length as usize)?;

            txt_data.append(&mut data);

            // Add 1 for the length byte itself, plus the length of the string
            bytes_read += 1 + length as u16;
        }

        Ok(TxtData { txt_data })
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
pub struct DnsHeader {
    pub id: u16,

    pub is_response: bool,
    pub op_code: u8,
    pub is_authoritative: bool,
    pub is_truncated: bool,
    pub is_recursion_desired: bool,
    pub is_recursion_avail: bool,
    pub z: u8,
    pub response_code: ResponseCode,

    pub question_count: u16,
    pub answer_count: u16,
    pub authoritative_count: u16,
    pub additional_count: u16,
}

impl DnsHeader {
    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;

        let is_response = (flags >> 15) & 1 != 0;
        let op_code = ((flags >> 11) & 0x0F) as u8;
        let is_authoritative = (flags >> 10) & 1 != 0;
        let is_truncated = (flags >> 9) & 1 != 0;
        let is_recursion_desired = (flags >> 8) & 1 != 0;
        let is_recursion_avail = (flags >> 7) & 1 != 0;
        let z = ((flags >> 4) & 0x07) as u8;
        let response_code = ResponseCode::from_number((flags & 0x0F) as u8);

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

#[derive(Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authoritatives: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
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
