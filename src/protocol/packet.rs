use crate::{MAX_PACKET_SIZE, buffer::PacketBuffer, protocol::{DnsHeader, DnsQuestion, DnsRecord, ResponseCode}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authoritatives: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn parse_from<const T: usize>(buf: [u8; T]) -> Result<Self, ResponseCode> {
        let mut packet_buf = PacketBuffer::<T>::from_raw_buffer(buf);

        let header = DnsHeader::read_from(&mut packet_buf)?;
        let mut questions: Vec<DnsQuestion> = Vec::with_capacity(header.question_count as usize);
        let mut answers: Vec<DnsRecord> = Vec::with_capacity(header.answer_count as usize);
        let mut authoritatives: Vec<DnsRecord> =
            Vec::with_capacity(header.authoritative_count as usize);
        let mut additionals: Vec<DnsRecord> = Vec::with_capacity(header.additional_count as usize);

        for _ in 0..header.question_count {
            let question = DnsQuestion::read_from(&mut packet_buf)?;
            questions.push(question);
        }

        for _ in 0..header.answer_count {
            let answer = DnsRecord::read_from(&mut packet_buf)?;
            answers.push(answer);
        }

        for _ in 0..header.authoritative_count {
            let authoritative = DnsRecord::read_from(&mut packet_buf)?;
            authoritatives.push(authoritative);
        }

        for _ in 0..header.additional_count {
            let additional = DnsRecord::read_from(&mut packet_buf)?;
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

    pub fn to_raw_bytes(&self) -> Option<[u8; MAX_PACKET_SIZE]> {
        None // TODO: impl.
    }
}
