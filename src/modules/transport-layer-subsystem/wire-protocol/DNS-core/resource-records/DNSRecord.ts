import type { DNS_CLASSES } from '../constants/DNS_CLASSES.js';
import type { DNS_TYPES } from '../constants/DNS_TYPES.js';
import type { RecordData } from './RDATA/RecordData.js';

export class DNSRecord {
  readonly name: string[];
  readonly type: DNS_TYPES;
  readonly rrClass: DNS_CLASSES;
  readonly ttl: number;
  readonly rdLength: number;
  readonly data: RecordData;

  constructor(name: string[], type: DNS_TYPES, rrClass: DNS_CLASSES, ttl: number, rdLength: number, data: RecordData) {
    this.name = name;
    this.type = type;
    this.rrClass = rrClass;
    this.ttl = ttl;
    this.rdLength = rdLength;
    this.data = data;
  }
}
