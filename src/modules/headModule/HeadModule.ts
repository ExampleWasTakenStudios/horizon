import type { Module } from '../Module.js';
import { TransportLayerSubsystem } from '../transportLayerSubsystem/TransportLayerSubsystem.js';

export class HeadModule implements Module {
  private transportLayerSubsystem: TransportLayerSubsystem;

  constructor() {
    this.transportLayerSubsystem = new TransportLayerSubsystem(true);
  }
}
