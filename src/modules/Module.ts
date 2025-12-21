import type { Logger } from '../logging/Logger.js';

export abstract class Module {
  protected logger: Logger;

  constructor(logger: Logger) {
    this.logger = logger;
  }
}
