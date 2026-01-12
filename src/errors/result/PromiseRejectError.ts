import { ResultError } from './ResultError.js';

export class PromiseRejectError extends ResultError {
  readonly err: unknown;

  constructor(err: unknown) {
    super('Promise rejected.');
    this.err = err;
  }
}
