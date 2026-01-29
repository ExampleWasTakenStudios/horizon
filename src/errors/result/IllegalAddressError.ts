import { ResultError } from './ResultError.js';

export class IllegalAddressError extends ResultError {
  public constructor(message: string, cause?: ResultError) {
    super(message, cause);
  }
}
