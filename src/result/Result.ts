import type { ResultError } from '../errors/result/ResultError.js';

export type TResult<T, E extends ResultError> = Success<T, never> | Failure<never, E>;

export abstract class Result<T, E extends ResultError> {
  abstract isSuccess(): this is Success<T, never>;
  abstract isFailure(): this is Failure<never, E>;

  static ok<T, E extends never>(value: T): Success<T, E> {
    return new Success(value);
  }

  static fail<T extends never, E extends ResultError>(error: E): Failure<T, E> {
    return new Failure<T, E>(error);
  }
}

export class Failure<T extends never, E extends ResultError> extends Result<T, E> {
  readonly error: E;

  constructor(error: E) {
    super();
    this.error = error;
  }

  override isSuccess(): this is Success<T, never> {
    return false;
  }
  override isFailure(): this is Failure<never, E> {
    return true;
  }
}

export class Success<T, E extends never> extends Result<T, E> {
  readonly value: T;

  constructor(value: T) {
    super();
    this.value = value;
  }

  override isSuccess(): this is Success<T, never> {
    return true;
  }

  override isFailure(): this is Failure<never, E> {
    return false;
  }
}
