import type { DNSHeader } from './DNSHeader.js';
import type { DNSQuestion } from './DNSQuestion.js';
import type { GenericResourceRecord } from './resource-records/ResourceRecord.js';

export class DNSPacket {
  private header: DNSHeader;
  private questions: DNSQuestion[];
  private answers: GenericResourceRecord[];
  private authority: GenericResourceRecord[];
  private additional: GenericResourceRecord[];

  constructor(
    header: DNSHeader,
    questions: DNSQuestion[],
    answers: GenericResourceRecord[],
    authority: GenericResourceRecord[],
    additional: GenericResourceRecord[]
  ) {
    this.header = header;
    this.questions = questions;
    this.answers = answers;
    this.authority = authority;
    this.additional = additional;
  }

  getHeader(): DNSHeader {
    return this.header;
  }

  getQuestions(): DNSQuestion[] {
    return this.questions;
  }

  getAnswers(): GenericResourceRecord[] {
    return this.answers;
  }

  getAuthority(): GenericResourceRecord[] {
    return this.authority;
  }

  getAdditional(): GenericResourceRecord[] {
    return this.additional;
  }
}
