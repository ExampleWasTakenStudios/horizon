import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import type { AData } from './AData.js';
import type { DomainName_Data } from './DomainNameData.js';
import type { MxData } from './MxData.js';
import type { OptData } from './OptData.js';
import type { SOA_Data } from './SOA_Data.js';
import type { TxtData } from './TxtData.js';

export abstract class RecordData {
  abstract readonly type: DNS_TYPES;

  isAData(): this is AData {
    return this.type === DNS_TYPES.A;
  }

  isNsData(): this is DomainName_Data {
    return this.type === DNS_TYPES.NS;
  }

  isCNameData(): this is DomainName_Data {
    return this.type === DNS_TYPES.CNAME;
  }

  isSoaData(): this is SOA_Data {
    return this.type === DNS_TYPES.SOA;
  }

  isMxData(): this is MxData {
    return this.type === DNS_TYPES.MX;
  }

  isTxtData(): this is TxtData {
    return this.type === DNS_TYPES.TXT;
  }

  isOptData(): this is OptData {
    return this.type === DNS_TYPES.OPT;
  }
}
