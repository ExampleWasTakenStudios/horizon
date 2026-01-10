interface ErrArgs<E> {
  error: E;
  debugInfo: unknown[];
}

interface ReturnState {
  err: boolean;
}

export interface ReturnSuccess<T> extends ReturnState {
  err: false;
  data: T;
}

export interface ReturnFailure<E> extends ReturnState {
  err: true;
  error: E;
  addDebugInfo: (...info: unknown[]) => this;
  debugInfo: unknown[];
}

export type ReturnResult<T, E> = ReturnSuccess<T> | ReturnFailure<E>;

/**
 * Constructs a {@link ReturnSuccess} object.
 */
export const ok = <T>(data: T): ReturnSuccess<T> => {
  return {
    err: false,
    data: data,
  };
};

/**
 * Constructs a {@link ReturnFailure} object.
 */
export const err = <E>({ error, debugInfo }: ErrArgs<E>): ReturnFailure<E> => {
  const failure: ReturnFailure<E> = {
    err: true,
    error: error,
    debugInfo: debugInfo,
    addDebugInfo: (...info: unknown[]) => {
      failure.debugInfo = debugInfo.concat(info);
      return failure;
    },
  };
  return failure;
};
