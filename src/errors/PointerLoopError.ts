export class PointerLoopError extends Error {
  constructor(message: string, options?: ErrorOptions) {
    super(message, options);
  }
}
