use crate::{
    buffer::PacketBuffer,
    protocol::{DnsHeader, DnsQuestion, DnsRecord, ResponseCode},
};

#[derive(Debug, Clone)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authoritatives: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn parse_from<const T: usize>(buffer: &mut PacketBuffer<T>) -> Result<Self, ResponseCode> {
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

    pub fn to_bytes<const T: usize>(&self, buffer: &mut PacketBuffer<T>) -> Result<usize, String> {
        if buffer.get_position() != 0 {
            return Err("Attempted to translate packet into non-empty buffer.".into());
        }

        let mut length_written = self.header.to_bytes(buffer)?;

        // Safety check since a DNS header must always be 12 bytes long
        if length_written != 12 {
            return Err(format!(
                "Header should be 12 bytes but was {length_written}"
            ));
        }

        for question in &self.questions {
            length_written += question.to_bytes(buffer)?;
        }

        for answer in &self.answers {
            length_written += answer.to_bytes(buffer)?;
        }

        for authoritative in &self.authoritatives {
            length_written += authoritative.to_bytes(buffer)?;
        }

        for additional in &self.additionals {
            length_written += additional.to_bytes(buffer)?;
        }

        Ok(length_written)
    }
}
