import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export interface EDNSOption {
  code: number;
  data: Buffer;
}

export class OptData extends RecordData {
  override readonly type: DNS_TYPES.OPT;
  readonly udpPayloadSize: number;
  readonly extendedRcode: number;
  readonly version: number;
  readonly doBit: boolean;
  readonly options: EDNSOption[];

  constructor(udpPayloadSize: number, extendedRcode: number, version: number, doBit: boolean, options: EDNSOption[]) {
    super();
    this.type = DNS_TYPES.OPT;
    this.udpPayloadSize = udpPayloadSize;
    this.extendedRcode = extendedRcode;
    this.version = version;
    this.doBit = doBit;
    this.options = options;
  }
}
