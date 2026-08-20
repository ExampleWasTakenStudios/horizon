use crate::{
    error::{DnsError, DnsResult},
    protocol::{DnsHeader, DnsQuestion, DnsRecord},
};

#[derive(Debug, Clone)]
pub struct DnsMessage {
    header: DnsHeader,
    question: DnsQuestion,
    answers: Vec<DnsRecord>,
    authoritatives: Vec<DnsRecord>,
    additionals: Vec<DnsRecord>,
}

impl DnsMessage {
    pub fn new(
        header: DnsHeader,
        question: DnsQuestion,
        answers: Vec<DnsRecord>,
        authoritatives: Vec<DnsRecord>,
        additionals: Vec<DnsRecord>,
    ) -> Self {
        Self {
            header,
            question,
            answers,
            authoritatives,
            additionals,
        }
    }

    pub fn get_header(&self) -> &DnsHeader {
        &self.header
    }

    pub fn get_header_mut(&mut self) -> &mut DnsHeader {
        &mut self.header
    }

    pub fn get_question(&self) -> &DnsQuestion {
        &self.question
    }

    pub fn get_question_mut(&mut self) -> &mut DnsQuestion {
        &mut self.question
    }

    pub fn get_answers(&self) -> &[DnsRecord] {
        &self.answers
    }

    pub fn get_answers_mut(&mut self) -> &mut [DnsRecord] {
        &mut self.answers
    }

    pub fn get_authoritatives(&self) -> &[DnsRecord] {
        &self.authoritatives
    }

    pub fn get_authoritatives_mut(&mut self) -> &mut [DnsRecord] {
        &mut self.authoritatives
    }

    pub fn get_additionals(&self) -> &[DnsRecord] {
        &self.additionals
    }

    pub fn get_additionals_mut(&mut self) -> &mut [DnsRecord] {
        &mut self.additionals
    }

    pub fn add_answer(&mut self, answer: DnsRecord) {
        self.header.answer_count += 1;
        self.answers.push(answer);
    }

    pub fn add_authoritative(&mut self, authoritative: DnsRecord) {
        self.header.authoritative_count += 1;
        self.authoritatives.push(authoritative);
    }

    pub fn add_additional(&mut self, additional: DnsRecord) {
        self.header.additional_count += 1;
        self.additionals.push(additional);
    }
}
