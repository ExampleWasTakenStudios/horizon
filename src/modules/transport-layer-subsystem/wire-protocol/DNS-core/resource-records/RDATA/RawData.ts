import type { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export class RawData extends RecordData {
  public override readonly type: DNS_TYPES;
  public readonly buffer: Buffer;

  public constructor(type: DNS_TYPES, buffer: Buffer) {
    super();
    this.type = type;
    this.buffer = buffer;
  }
}
