export abstract class ResultError {
  readonly message: string;
  readonly cause: ResultError | undefined;

  constructor(message: string, cause?: ResultError) {
    this.message = message;
    this.cause = cause;
  }

  getOriginalError(): ResultError {
    if (this.cause) {
      return this.cause.getOriginalError();
    }
    return this;
  }

  getChain(): ResultError[] {
    const chain: ResultError[] = [this];
    let current = this.cause;
    while (current) {
      chain.push(current);
      current = current.cause;
    }
    return chain;
  }

  toString(): string {
    return this.message;
  }
}
