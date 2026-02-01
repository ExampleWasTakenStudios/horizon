import type { ConfigManager } from '@src/config/ConfigManager.js';
import type { Logger } from '@src/logging/Logger.js';

export abstract class Subsystem {
  protected logger: Logger;
  protected config: ConfigManager;

  public constructor(logger: Logger, config: ConfigManager) {
    this.logger = logger;
    this.config = config;
  }

  /**
   * Start the module.
   *
   * This method should enable the module to start performing its business logic.
   */
  public abstract start(): void;

  /**
   * Stops the module.
   *
   * This method should cause the module to terminate all its operation for a graceful shutdown.
   */
  public abstract stop(): void;
}
