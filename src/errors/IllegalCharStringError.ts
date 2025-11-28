export class IllegalCharStringError extends Error {
  constructor(message: string, options?: ErrorOptions) {
    super(message, options);
  }
}
