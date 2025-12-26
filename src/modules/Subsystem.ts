import type { Logger } from '../logging/Logger.js';
import type { ConfigManager } from './config-module/ConfigModule.js';

export abstract class Subsystem {
  protected logger: Logger;
  protected config: ConfigManager;

  constructor(logger: Logger, config: ConfigManager) {
    this.logger = logger;
    this.config = config;
  }
}
