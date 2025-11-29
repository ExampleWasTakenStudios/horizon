import dgram, { Socket, type RemoteInfo } from 'node:dgram';
import type { AddressInfo } from 'node:net';
import type { NetworkInterfaceIPv4 } from '../../types/NetworkInterfaceInfo.js';
import type { Module } from '../Module.js';
import { WireProtocolModule } from '../wire-protocol/WireProtocolModule.js';
import { TransportLayerSubsystem } from './TransportLayerSubsystem.js';

export class DownstreamModule implements Module {
  private socket: Socket;
  private wireProtocolModule: WireProtocolModule;
  private networkInterfaceIPv4: NetworkInterfaceIPv4;

  constructor(networkInterfaceIPv4: NetworkInterfaceIPv4) {
    this.socket = dgram.createSocket({ type: 'udp4' });

    this.socket.on('error', (error) => this.onError(error));
    this.socket.on('close', () => this.onClose());
    this.socket.on('listening', () => this.onListening());
    this.socket.on('message', (msg, rinfo) => this.onMessage(msg, rinfo));

    this.wireProtocolModule = new WireProtocolModule();
    this.networkInterfaceIPv4 = networkInterfaceIPv4;
  }

  /**
   * @throws Illegal IP address error when an invalid IP address is passed.
   * @throws Illegal port error when a port < 0 || > 65535 is passed.
   */
  send(msg: Buffer, destAddress: string, destPort: number): void {
    if (TransportLayerSubsystem.IP_ADDRESS_REGEX.test(destAddress)) {
      throw new Error(`Illegal IP address received: ${destAddress}`);
    }

    if (destPort < 0 || destPort > 65535) {
      throw new Error('Illegal port received. Must be between 0-65535');
    }

    this.socket.send(msg, 0, msg.length, destPort, destAddress);
  }

  bind(): void {
    this.socket.bind(53, this.networkInterfaceIPv4.address);
  }

  close(): void {
    this.socket.close();
  }

  getAddressInfo(): AddressInfo {
    return this.socket.address();
  }

  private onMessage(msg: Buffer, rinfo: RemoteInfo): void {
    const decodedMsg = this.wireProtocolModule.decode(msg);

    console.log('Received msg: ', JSON.stringify(decodedMsg, null, 2), ' from ', rinfo);
  }

  private onListening(): void {
    const address = this.socket.address();
    console.log(`Server listening on ${address.address}:${address.port}.`);
  }

  private onClose(): void {
    console.log('Server closed.');
  }

  private onError(error: Error): void {
    console.error('Server Error: ', error);
  }
}
