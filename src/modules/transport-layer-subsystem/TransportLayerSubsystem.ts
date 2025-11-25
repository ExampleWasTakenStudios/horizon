import type { NetworkInterfaceIPv4 } from '../../types/NetworkInterfaceInfo.js';
import type { Subsystem } from '../Subsystem.js';
import { DownstreamModule } from './DownstreamModule.js';
import { UpstreamModule } from './UpstreamModule.js';
import os from 'node:os';

export class TransportLayerSubsystem implements Subsystem {
  private downstreamModule: DownstreamModule;
  private upstreamModule: UpstreamModule;

  constructor(bindToLocalhost?: boolean) {
    this.downstreamModule = new DownstreamModule();
    this.upstreamModule = new UpstreamModule();

    const interfaces = this.getNormalizedIPv4Interfaces();
    console.log('Discorvered ', interfaces.length, ' network interfaces.');

    if (bindToLocalhost) {
      console.log('bindToLocalhost enabled -> binding to localhost');
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
