import type { DNS_RESPONSE_CODES } from '../../modules/resolution-subsystem/wire-protocol-module/DNS-core/constants/DNS_RESPONSE_CODES.js';
import { ResultError } from './ResultError.js';

export class DNSParseError extends ResultError {
  public readonly rCode: DNS_RESPONSE_CODES;

  public constructor(message: string, rCode: DNS_RESPONSE_CODES) {
    super(message);
    this.rCode = rCode;
  }
}
