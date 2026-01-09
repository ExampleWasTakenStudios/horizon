import type { DNS_RESPONSE_CODES } from '../modules/transport-layer-subsystem/wire-protocol/DNS-core/constants/DNS_RESPONSE_CODES.js';
import type { ReturnFailure, ReturnSuccess } from '../types/ReturnResult.js';

/**
 * Constructs a {@link ReturnSuccess} object.
 */
export const ok = <T>(data: T): ReturnSuccess<T> => {
  return {
    success: true,
    data: data,
  };
};

interface ErrArgs {
  rCode: DNS_RESPONSE_CODES;
  debugInfo: unknown;
}

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
