import type { Logger } from '../logging/Logger.js';

export abstract class Subsystem {
  protected logger: Logger;

  constructor(logger: Logger) {
    this.logger = logger;
  }
}
