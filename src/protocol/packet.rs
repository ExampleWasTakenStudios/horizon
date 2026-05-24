use crate::{
    buffer::PacketBuffer,
    protocol::{DnsHeader, DnsQuestion, DnsRecord, ResponseCode},
};

#[derive(Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authoritatives: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn create(
        header: DnsHeader,
        questions: Option<Vec<DnsQuestion>>,
        answers: Option<Vec<DnsRecord>>,
        authoritatives: Option<Vec<DnsRecord>>,
        additionals: Option<Vec<DnsRecord>>,
    ) -> DnsPacket {
        DnsPacket {
            header,
            questions: questions.unwrap_or(Vec::with_capacity(0)),
            answers: answers.unwrap_or(Vec::with_capacity(0)),
            authoritatives: authoritatives.unwrap_or(Vec::with_capacity(0)),
            additionals: additionals.unwrap_or(Vec::with_capacity(0)),
        }
    }

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

    pub fn to_vec(&self) -> [u8; 512] {
        let buffer: Vec<u8> = Vec::with_capacity(12);

        buffer.extend(self.header.to_vec());

        for question in self.questions {
            buffer.extend(question.to_vec());
        }

        for answer in self.answers {
            buffer.extend(answer)
        }
    }
}
