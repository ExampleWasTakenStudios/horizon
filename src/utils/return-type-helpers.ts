import type { DNS_RESPONSE_CODES } from '../modules/transport-layer-subsystem/wire-protocol/DNS-core/constants/DNS_RESPONSE_CODES.js';

interface ErrArgs {
  rCode: DNS_RESPONSE_CODES;
  debugInfo: unknown;
}

export interface ReturnSuccess<T> {
  success: true;
  data: T;
}

export interface ReturnFailure {
  success: false;
  rCode: DNS_RESPONSE_CODES;
  debugInfo: unknown;
}

export type ReturnResult<T> = ReturnSuccess<T> | ReturnFailure;

/**
 * Constructs a {@link ReturnSuccess} object.
 */
export const ok = <T>(data: T): ReturnSuccess<T> => {
  return {
    success: true,
    data: data,
  };
};

/**
 * Constructs a {@link ReturnFailure} object.
 */
export const err = ({ rCode, debugInfo }: ErrArgs): ReturnFailure => {
  return {
    success: false,
    rCode: rCode,
    debugInfo: debugInfo,
  };
};
