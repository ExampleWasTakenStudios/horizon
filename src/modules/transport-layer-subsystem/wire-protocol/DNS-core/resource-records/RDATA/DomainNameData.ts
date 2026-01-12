import type { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export class DomainName_Data extends RecordData {
  override readonly type: DNS_TYPES;
  readonly domainName: string[];

  constructor(type: DNS_TYPES, domainName: string[]) {
    super();
    this.type = type;
    this.domainName = domainName;
  }
}
