import os from 'node:os';
import type { ConfigManager } from '../../config/ConfigManager.js';
import type { Logger } from '../../logging/Logger.js';
import type { NetworkInterfaceIPv4 } from '../../types/NetworkInterfaceInfo.js';
import { Subsystem } from '../Subsystem.js';
import { NetworkModule } from './NetworkModule.js';

export class TransportLayerSubsystem extends Subsystem {
  private readonly downstreamModule: NetworkModule;
  private readonly upstreamModule: NetworkModule;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.downstreamModule = new NetworkModule(this.logger.spawnSubLogger('DOWNSTREAM MODULE'), this.config);

    this.upstreamModule = new NetworkModule(this.logger.spawnSubLogger('UPSTREAM MODULE'), this.config);

    const interfaces = this.getNormalizedIPv4Interfaces();

    this.logger.info('Discovered ', interfaces.length, ' network interfaces.');

    // Bind modules
    this.logger.info('Binding modules...');

    const downstreamDnsIPv4Address = this.config.getConfig().transportLayerSubsystem.downstreamModule.dnsIPv4Address;

    this.downstreamModule.bind(downstreamDnsIPv4Address, 53);
    // The upstream module does not bind to a specific port -> uses random port per request
  }

  public getDownstreamModule(): NetworkModule {
    return this.downstreamModule;
  }

  public getUpstreamModule(): NetworkModule {
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
