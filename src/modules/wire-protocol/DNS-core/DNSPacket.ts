import type { DNSHeader } from './DNSHeader.js';
import type { DNSQuestion } from './DNSQuestion.js';
import type { ResourceRecord } from './ResourceRecord.js';

export class DNSPacket {
  private header: DNSHeader;
  private questions: DNSQuestion[];
  private answers: ResourceRecord[];
  private authority: ResourceRecord[];
  private additional: ResourceRecord[];

  constructor(
    header: DNSHeader,
    questions: DNSQuestion[],
    answers: ResourceRecord[],
    authority: ResourceRecord[],
    additional: ResourceRecord[]
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

  getAnswers(): ResourceRecord[]{
    return this.answers;
  }

  getAuthority(): ResourceRecord[] {
    return this.authority;
  }

  getAdditional(): ResourceRecord[] {
    return this.additional;
  }
}
