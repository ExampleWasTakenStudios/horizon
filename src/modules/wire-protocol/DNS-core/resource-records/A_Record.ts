import type { DNS_CLASSES } from '../constants/DNS_CLASSES.js';
import type { DNS_TYPES } from '../constants/DNS_TYPES.js';
import type { RDataMap } from './RDataMap.js';
import type { ResourceRecord } from './ResourceRecord.js';

export class A_Record implements ResourceRecord<RDataMap[DNS_TYPES.A]> {
  readonly name: string;
  readonly type: DNS_TYPES.A;
  readonly RR_class: DNS_CLASSES;
  readonly ttl: number;
  readonly rdLength: number;
  readonly rData: RDataMap[DNS_TYPES.A];

  constructor(
    name: string,
    type: DNS_TYPES.A,
    RR_class: DNS_CLASSES,
    ttl: number,
    rdLength: number,
    rData: RDataMap[DNS_TYPES.A]
  ) {
    this.name = name;
    this.type = type;
    this.RR_class = RR_class;
    this.ttl = ttl;
    this.rdLength = rdLength;
    this.rData = rData;
  }
}
