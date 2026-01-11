import type { DNS_RESPONSE_CODES } from '../../modules/transport-layer-subsystem/wire-protocol/DNS-core/constants/DNS_RESPONSE_CODES.js';
import { ResultError } from './ResultError.js';

export class DNSParseError extends ResultError {
  readonly rCode: DNS_RESPONSE_CODES;

  constructor(message: string, rCode: DNS_RESPONSE_CODES) {
    super(message);
    this.rCode = rCode;
  }
}
