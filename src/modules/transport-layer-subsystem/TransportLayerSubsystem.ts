import os from 'node:os';
import type { ConfigManager } from '../../config/ConfigManager.js';
import type { Logger } from '../../logging/Logger.js';
import type { NetworkInterfaceIPv4 } from '../../types/NetworkInterfaceInfo.js';
import { Subsystem } from '../Subsystem.js';
import { DownstreamModule } from './DownstreamModule.js';
import { UpstreamModule } from './UpstreamModule.js';

export const IPv4_ADDRESS_REGEX =
  /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

export class TransportLayerSubsystem extends Subsystem {
  private readonly downstreamModule: DownstreamModule;
  private readonly upstreamModule: UpstreamModule;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.downstreamModule = new DownstreamModule(this.logger.spawnSubLogger('DOWNSTREAM MODULE'), config);
    this.upstreamModule = new UpstreamModule(this.logger.spawnSubLogger('UPSTREAM MODULE'), config);

    const interfaces = this.getNormalizedIPv4Interfaces();

    this.logger.info('Discovered ', interfaces.length, ' network interfaces.');

    // Bind modules
    this.logger.info('Binding modules...');
    this.downstreamModule.bind();
  }

  public getDownstreamModule(): DownstreamModule {
    return this.downstreamModule;
  }

  public getUpstreamModule(): UpstreamModule {
    return this.upstreamModule;
  }

  private getNormalizedIPv4Interfaces(): NetworkInterfaceIPv4[] {
    const raw = os.networkInterfaces();
    const normalized: NetworkInterfaceIPv4[] = [];

    for (const [name, entries] of Object.entries(raw)) {
      if (!entries) continue;

      for (const currentEntry of entries) {
        if (currentEntry.family === 'IPv4') {
          normalized.push({ name, ...currentEntry });
        }
      }
    }

    return normalized;
  }
}
