import type { DNS_TYPES } from '../../constants/DNS_TYPES.js';

export abstract class RecordData {
  abstract readonly type: DNS_TYPES;
}
