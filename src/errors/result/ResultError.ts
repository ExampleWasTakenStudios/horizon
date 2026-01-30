export abstract class ResultError {
  public readonly message: string;
  public readonly cause: ResultError | undefined;

  public constructor(message: string, cause?: ResultError) {
    this.message = message;
    this.cause = cause;
  }

  public getOriginalError(): ResultError {
    if (this.cause) {
      return this.cause.getOriginalError();
    }
    return this;
  }

  public getChain(): ResultError[] {
    const chain: ResultError[] = [this];
    let current = this.cause;
    while (current) {
      chain.push(current);
      current = current.cause;
    }
    return chain;
  }

  public toString(): string {
    return this.message;
  }
}
