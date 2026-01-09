import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export class TxtData extends RecordData {
  override readonly type: DNS_TYPES.TXT;
  readonly text: string[];
  constructor(text: string[]) {
    super();
    this.type = DNS_TYPES.TXT;
    this.text = text;
  }
}
