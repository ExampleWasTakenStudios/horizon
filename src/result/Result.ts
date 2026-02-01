import { PromiseRejectError } from '@src/errors/result/PromiseRejectError.js';
import type { ResultError } from '@src/errors/result/ResultError.js';

export type TResult<T, E extends ResultError> = Success<T, never> | Failure<never, E>;

export abstract class Result<T, E extends ResultError> {
  public abstract isSuccess(): this is Success<T, never>;
  public abstract isFailure(): this is Failure<never, E>;

  public static async fromPromise<T>(promise: Promise<T>): Promise<TResult<T, PromiseRejectError>> {
    try {
      return Result.ok(await promise);
    } catch (err) {
      return Result.fail(new PromiseRejectError(err));
    }
  }

  public static ok<T, E extends never>(value: T): Success<T, E> {
    return new Success(value);
  }

  public static fail<T extends never, E extends ResultError>(error: E): Failure<T, E> {
    return new Failure<T, E>(error);
  }
}

export class Failure<T extends never, E extends ResultError> extends Result<T, E> {
  public readonly error: E;

  public constructor(error: E) {
    super();
    this.error = error;
  }

  public override isSuccess(): this is Success<T, never> {
    return false;
  }

  public override isFailure(): this is Failure<never, E> {
    return true;
  }
}

export class Success<T, E extends never> extends Result<T, E> {
  public readonly value: T;

  public constructor(value: T) {
    super();
    this.value = value;
  }

  public override isSuccess(): this is Success<T, never> {
    return true;
  }

  public override isFailure(): this is Failure<never, E> {
    return false;
  }
}
