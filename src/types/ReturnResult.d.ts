import { type DNS_RESPONSE_CODES } from '../modules/transport-layer-subsystem/wire-protocol/DNS-core/constants/DNS_RESPONSE_CODES.ts';

export interface ReturnSuccess<T> {
  success: true;
  data: T;
}

export interface ReturnFailure {
  success: false;
  rCode: DNS_RESPONSE_CODES;
}

export type ReturnResult<T> = ReturnSuccess<T> | ReturnFailure;
