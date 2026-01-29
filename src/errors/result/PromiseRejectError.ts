import { ResultError } from './ResultError.js';

export class PromiseRejectError extends ResultError {
  public readonly err: unknown;

  public constructor(err: unknown) {
    super('Promise rejected.');
    this.err = err;
  }
}
