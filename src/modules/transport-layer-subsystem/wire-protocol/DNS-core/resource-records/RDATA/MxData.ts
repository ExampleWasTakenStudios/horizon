import type { uint16 } from '../../../../../../types/number-types.js';
import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export class MxData extends RecordData {
  override readonly type: DNS_TYPES.MX;
  readonly preference: uint16;
  readonly exchange: string[];

  constructor(preference: uint16, exchange: string[]) {
    super();
    this.type = DNS_TYPES.MX;
    this.preference = preference;
    this.exchange = exchange;
  }
}
