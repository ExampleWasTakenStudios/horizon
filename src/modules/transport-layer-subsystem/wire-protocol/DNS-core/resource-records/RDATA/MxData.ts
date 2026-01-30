import type { uint16 } from '../../../../../../types/number-types.js';
import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export class MxData extends RecordData {
  public override readonly type: DNS_TYPES.MX;
  public readonly preference: uint16;
  public readonly exchange: string[];

  public constructor(preference: uint16, exchange: string[]) {
    super();
    this.type = DNS_TYPES.MX;
    this.preference = preference;
    this.exchange = exchange;
  }
}
