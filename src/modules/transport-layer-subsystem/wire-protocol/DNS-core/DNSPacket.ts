import type { DNS_TYPES } from './constants/DNS_TYPES.js';
import { DNSHeader } from './DNSHeader.js';
import type { DNSQuestion } from './DNSQuestion.js';
import type { RDataMap } from './resource-records/RDataMap.js';
import type { ResourceRecord } from './resource-records/ResourceRecord.js';

export class DNSPacket {
  private readonly header: DNSHeader;
  private readonly questions: DNSQuestion[];
  private readonly answers: ResourceRecord<RDataMap[DNS_TYPES]>[];
  private readonly authority: ResourceRecord<RDataMap[DNS_TYPES]>[];
  private readonly additional: ResourceRecord<RDataMap[DNS_TYPES]>[];

  constructor(
    header: DNSHeader,
    questions: DNSQuestion[],
    answers: ResourceRecord<RDataMap[DNS_TYPES]>[],
    authoritative: ResourceRecord<RDataMap[DNS_TYPES]>[],
    additional: ResourceRecord<RDataMap[DNS_TYPES]>[]
  ) {
    this.header = header;
    this.questions = questions;
    this.answers = answers;
    this.authority = authoritative;
    this.additional = additional;
  }

  getHeader(): DNSHeader {
    return this.header;
  }

  getQuestions(): DNSQuestion[] {
    return this.questions;
  }

  getAnswers(): ResourceRecord<RDataMap[DNS_TYPES]>[] {
    return this.answers;
  }

  getAuthority(): ResourceRecord<RDataMap[DNS_TYPES]>[] {
    return this.authority;
  }

  getAdditional(): ResourceRecord<RDataMap[DNS_TYPES]>[] {
    return this.additional;
  }
}
