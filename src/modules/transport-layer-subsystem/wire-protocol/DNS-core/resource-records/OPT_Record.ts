import type { DNS_TYPES } from '../constants/DNS_TYPES.js';
import type { RDataMap } from './RDataMap.js';
import type { I_OPT_Record } from './ResourceRecord.js';

export class OPT_Record implements I_OPT_Record {
  /**
   * The requestors maximum UDP payload size
   */
  readonly RR_class: number;
  /**
   * MUST be 0 (root domain)
   */
  readonly name: string;
  readonly type: DNS_TYPES.OPT;
  /**
   * The extended RCODE and VERSION number. The individual values are also available as separate properties.
   */
  readonly ttl: number;
  readonly rdLength: number;
  /**
   * We are not supporting any RDATA format at the moment - hence this is unknown and we simply skip RDLENGTH bytes.
   */
  readonly rData: RDataMap[DNS_TYPES.OPT];
  readonly extendedRCode: number;
  readonly ednsVersion: number;

  constructor(
    name: string,
    type: DNS_TYPES.OPT,
    ttl: number,
    RR_class: number,
    rdLength: number,
    rData: RDataMap[DNS_TYPES.OPT],
    extendedRCode: number,
    ednsVersion: number
  ) {
    this.name = name;
    this.type = type;
    this.ttl = ttl;
    this.RR_class = RR_class;
    this.rdLength = rdLength;
    this.rData = rData;
    this.extendedRCode = extendedRCode;
    this.ednsVersion = ednsVersion;
  }
}
