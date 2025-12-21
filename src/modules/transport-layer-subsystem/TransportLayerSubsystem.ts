import os from 'node:os';
import type { Logger } from '../../logging/Logger.js';
import type { NetworkInterfaceIPv4 } from '../../types/NetworkInterfaceInfo.js';
import { Subsystem } from '../Subsystem.js';
import { DownstreamModule } from './DownstreamModule.js';
import { UpstreamModule } from './UpstreamModule.js';

export class TransportLayerSubsystem extends Subsystem {
  private downstreamModule: DownstreamModule;
  private upstreamModule: UpstreamModule;

  constructor(logger: Logger, bindToLocalhost?: boolean) {
    super(logger);

    this.downstreamModule = new DownstreamModule(this.logger.getSubLogger('DOWNSTREAM MODULE'));
    this.upstreamModule = new UpstreamModule(this.logger.getSubLogger('UPSTREAM MODULE'));

    const interfaces = this.getNormalizedIPv4Interfaces();

    this.logger.info('Discorvered ', interfaces.length, ' network interfaces.');

    if (bindToLocalhost) {
      this.logger.warn('bindToLocalhost enabled -> binding to localhost');
      this.downstreamModule.bind('127.0.0.1');
    }
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
