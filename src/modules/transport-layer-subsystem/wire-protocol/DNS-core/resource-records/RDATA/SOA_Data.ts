import type { uint32 } from '../../../../../../types/number-types.js';
import { DNS_TYPES } from '../../constants/DNS_TYPES.js';
import { RecordData } from './RecordData.js';

export class SOA_Data extends RecordData {
  override readonly type: DNS_TYPES.SOA;
  readonly mName: string[];
  readonly rName: string[];
  readonly serial: uint32;
  readonly refresh: uint32;
  readonly retry: uint32;
  readonly expire: uint32;
  readonly minimum: uint32;

  constructor(
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
