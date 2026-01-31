import os from 'node:os';
import type { ConfigManager } from '../../config/ConfigManager.js';
import type { Logger } from '../../logging/Logger.js';
import type { NetworkInterfaceIPv4 } from '../../types/NetworkInterfaceInfo.js';
import { Subsystem } from '../Subsystem.js';
import { NetworkModule } from './NetworkModule.js';

export const IPv4_ADDRESS_REGEX =
  /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

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
    const upstreamDnsIPv4Address = this.config.getConfig().transportLayerSubsystem.upstreamModule.dnsIPv4Address;
    const upstreamDnsIPv4Port = this.config.getConfig().transportLayerSubsystem.upstreamModule.dnsIPv4Port;

    this.downstreamModule.bind(downstreamDnsIPv4Address, 53);
    this.upstreamModule.bind(upstreamDnsIPv4Address, upstreamDnsIPv4Port);
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
