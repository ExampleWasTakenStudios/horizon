import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import type { AData } from './AData.js';
import type { DomainName_Data } from './DomainNameData.js';
import type { MxData } from './MxData.js';
import type { OptData } from './OptData.js';
import type { SOA_Data } from './SOA_Data.js';
import type { TxtData } from './TxtData.js';

export abstract class RecordData {
  public abstract readonly type: DNS_TYPES;

  public isAData(): this is AData {
    return this.type === DNS_TYPES.A;
  }

  public isNsData(): this is DomainName_Data {
    return this.type === DNS_TYPES.NS;
  }

  public isCNameData(): this is DomainName_Data {
    return this.type === DNS_TYPES.CNAME;
  }

  public isSoaData(): this is SOA_Data {
    return this.type === DNS_TYPES.SOA;
  }

  public isMxData(): this is MxData {
    return this.type === DNS_TYPES.MX;
  }

  public isTxtData(): this is TxtData {
    return this.type === DNS_TYPES.TXT;
  }

  public isOptData(): this is OptData {
    return this.type === DNS_TYPES.OPT;
  }
}
