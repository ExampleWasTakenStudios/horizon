import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import type { AData } from './AData.js';
import type { DomainNameData } from './DomainNameData.js';
import type { MxData } from './MxData.js';
import type { OptData } from './OptData.js';
import type { SoaData } from './SoaData.js';
import type { TxtData } from './TxtData.js';

export abstract class RecordData {
  public abstract readonly type: DNS_TYPES;

  public isAData(): this is AData {
    return this.type === DNS_TYPES.A;
  }

  public isNsData(): this is DomainNameData {
    return this.type === DNS_TYPES.NS;
  }

  public isCNameData(): this is DomainNameData {
    return this.type === DNS_TYPES.CNAME;
  }

  public isSoaData(): this is SoaData {
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
