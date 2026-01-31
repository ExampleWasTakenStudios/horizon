import type { ResultError } from '../../errors/result/ResultError.js';
import type { TResult } from '../../result/Result.js';

export interface PendingRequest {
  resolve(value: TResult<Buffer, ResultError>): void;
  timer: NodeJS.Timeout;
}
