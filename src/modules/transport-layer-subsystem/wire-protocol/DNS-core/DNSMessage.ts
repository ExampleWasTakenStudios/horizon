import { DNSHeader } from './DNSHeader.js';
import type { DNSQuestion } from './DNSQuestion.js';
import type { DNSRecord } from './resource-records/DNSRecord.js';

export class DNSMessage {
  private readonly header: DNSHeader;
  private readonly questions: DNSQuestion[];
  private readonly answers: DNSRecord[];
  private readonly authority: DNSRecord[];
  private readonly additional: DNSRecord[];

  public constructor(
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

  public getHeader(): Readonly<DNSHeader> {
    return this.header;
  }

  public getQuestions(): readonly DNSQuestion[] {
    return this.questions;
  }

  public getAnswers(): readonly DNSRecord[] {
    return this.answers;
  }

  public getAuthority(): readonly DNSRecord[] {
    return this.authority;
  }

  public getAdditional(): readonly DNSRecord[] {
    return this.additional;
  }
}
