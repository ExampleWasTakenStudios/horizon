export class IllegalRDataFieldError extends Error {
  constructor(message: string, options?: ErrorOptions) {
    super(message, options);
  }
}
