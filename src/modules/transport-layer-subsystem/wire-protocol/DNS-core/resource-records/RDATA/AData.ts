import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export class AData extends RecordData {
  public override type: DNS_TYPES.A;
  public readonly address: string;

  public constructor(address: string) {
    super();
    this.type = DNS_TYPES.A;
    this.address = address;
  }
}
