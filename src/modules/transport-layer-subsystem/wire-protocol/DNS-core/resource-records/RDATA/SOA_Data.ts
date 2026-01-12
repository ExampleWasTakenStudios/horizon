import type { uint32 } from '../../../../../../types/number-types.js';
import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export class SOA_Data extends RecordData {
  public override readonly type: DNS_TYPES.SOA;
  public readonly mName: string[];
  public readonly rName: string[];
  public readonly serial: uint32;
  public readonly refresh: uint32;
  public readonly retry: uint32;
  public readonly expire: uint32;
  public readonly minimum: uint32;

  public constructor(
    mName: string[],
    rName: string[],
    serial: uint32,
    refresh: uint32,
    retry: uint32,
    expire: uint32,
    minimum: uint32
  ) {
    super();
    this.type = DNS_TYPES.SOA;
    this.mName = mName;
    this.rName = rName;
    this.serial = serial;
    this.refresh = refresh;
    this.retry = retry;
    this.expire = expire;
    this.minimum = minimum;
  }
}
