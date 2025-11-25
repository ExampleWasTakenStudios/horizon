import type { NetworkInterfaceInfoIPv4 } from 'node:os';

export interface NetworkInterfaceIPv4 extends NetworkInterfaceInfoIPv4 {
  name: string;
}
