import { DNSHeader } from './DNSHeader.js';
import type { DNSQuestion } from './DNSQuestion.js';
import type { DNSRecord } from './resource-records/DNSRecord.js';

export class DNSMessage {
  private readonly header: DNSHeader;
  private readonly questions: DNSQuestion[];
  private readonly answers: DNSRecord[];
  private readonly authority: DNSRecord[];
  private readonly additional: DNSRecord[];

  constructor(
    header: DNSHeader,
    questions: DNSQuestion[],
    answers: DNSRecord[],
    authoritative: DNSRecord[],
    additional: DNSRecord[]
  ) {
    this.header = header;
    this.questions = questions;
    this.answers = answers;
    this.authority = authoritative;
    this.additional = additional;
  }

  getHeader(): Readonly<DNSHeader> {
    return this.header;
  }

  getQuestions(): Readonly<DNSQuestion[]> {
    return this.questions;
  }

  getAnswers(): Readonly<DNSRecord[]> {
    return this.answers;
  }

  getAuthority(): Readonly<DNSRecord[]> {
    return this.authority;
  }

  getAdditional(): Readonly<DNSRecord[]> {
    return this.additional;
  }
}
