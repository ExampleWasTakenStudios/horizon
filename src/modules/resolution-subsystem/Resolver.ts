import type { ResultError } from '../../errors/result/ResultError.js';
import type { TResult } from '../../result/Result.js';

export interface Resolver {
  resolveQuery(query: Buffer): Promise<TResult<Buffer, ResultError>>;
}
