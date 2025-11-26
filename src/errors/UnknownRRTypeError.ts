export class UnknownRRTypeError extends Error {
  constructor(message: string, options?: ErrorOptions) {
    super(message, options);
  }
}
