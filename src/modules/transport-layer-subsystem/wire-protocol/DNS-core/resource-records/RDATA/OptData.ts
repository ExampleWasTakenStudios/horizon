import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export interface EDNSOption {
  code: number;
  data: Buffer;
}

export class OptData extends RecordData {
  public override readonly type: DNS_TYPES.OPT;
  public readonly udpPayloadSize: number;
  public readonly extendedRcode: number;
  public readonly version: number;
  public readonly doBit: boolean;
  public readonly options: EDNSOption[];

  public constructor(
    udpPayloadSize: number,
    extendedRcode: number,
    version: number,
    doBit: boolean,
    options: EDNSOption[]
  ) {
    super();
    this.type = DNS_TYPES.OPT;
    this.udpPayloadSize = udpPayloadSize;
    this.extendedRcode = extendedRcode;
    this.version = version;
    this.doBit = doBit;
    this.options = options;
  }
}
