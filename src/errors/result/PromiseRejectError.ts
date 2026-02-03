import { ResultError } from './ResultError.js';

export class PromiseRejectError extends ResultError {
  public readonly error: Error;

  public constructor(error: Error) {
    super(error.message);
    this.error = error;
  }
}
