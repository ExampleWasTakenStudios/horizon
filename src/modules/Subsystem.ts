import type { ConfigManager } from '../config/ConfigManager.js';
import type { Logger } from '../logging/Logger.js';

export abstract class Subsystem {
  protected logger: Logger;
  protected config: ConfigManager;

  constructor(logger: Logger, config: ConfigManager) {
    this.logger = logger;
    this.config = config;
  }
}
