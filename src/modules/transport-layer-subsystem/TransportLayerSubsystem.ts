import os from 'node:os';
import type { NetworkInterfaceIPv4 } from '../../types/NetworkInterfaceInfo.js';
import type { Subsystem } from '../Subsystem.js';
import { DownstreamModule } from './DownstreamModule.js';
import { UpstreamModule } from './UpstreamModule.js';

export class TransportLayerSubsystem implements Subsystem {
  static readonly IP_ADDRESS_REGEX =
    /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

  private downstreamModule: DownstreamModule;
  private upstreamModule: UpstreamModule;

  constructor() {
    const interfaces = this.getNormalizedIPv4Interfaces();

    this.downstreamModule = new DownstreamModule(interfaces.find((current) => current.address === '192.168.1.2')!); // TODO: this should be based on configuration
    this.upstreamModule = new UpstreamModule();

    this.downstreamModule.bind();
  }

  getDownstreamModule(): DownstreamModule {
    return this.downstreamModule;
  }

  getUpstreamModule(): UpstreamModule {
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
