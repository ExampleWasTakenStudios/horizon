import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export class AData extends RecordData {
  override type: DNS_TYPES.A;
  readonly address: string;

  constructor(address: string) {
    super();
    this.type = DNS_TYPES.A;
    this.address = address;
  }
}
