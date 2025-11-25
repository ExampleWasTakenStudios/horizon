import type { AddressInfo } from 'node:net';
import type { Module } from '../Module.js';
import dgram, { Socket, type RemoteInfo } from 'node:dgram';
import { WireProtocolModule } from '../wire-protocol/WireProtocolModule.js';

const IP_ADDRESS_REGEX =
  /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

export class DownstreamModule implements Module {
  private socket: Socket;
  private wireProtocolModule: WireProtocolModule;

  constructor() {
    this.socket = dgram.createSocket({ type: 'udp4' });

    this.socket.on('error', (error) => this.onError(error));
    this.socket.on('close', () => this.onClose());
    this.socket.on('listening', () => this.onListening());
    this.socket.on('message', (msg, rinfo) => this.onMessage(msg, rinfo));

    this.wireProtocolModule = new WireProtocolModule();
  }

  /**
   * @throws Illegal IP address error when an invalid IP address is passed.
   * @throws Illegal port error when a port < 0 || > 65535 is passed.
   */
  send(msg: Buffer, address: string, port: number): void {
    if (IP_ADDRESS_REGEX.test(address)) {
      throw new Error(`Illegal IP address received: ${address}`);
    }

    if (port < 0 || port > 65535) {
      throw new Error('Illegal port received. Must be between 0-65535');
    }

    this.socket.send(msg, 0, msg.length, port, address);
  }

  bind(address: string): void {
    this.socket.bind(53, address);
  }

  close(): void {
    this.socket.close();
  }

  getAddressInfo(): AddressInfo {
    return this.socket.address();
  }

  private onMessage(msg: Buffer, rinfo: RemoteInfo): void {
    const decodedMsg = this.wireProtocolModule.decode(msg);

    console.log(
      'Received msg: ',
      JSON.stringify(decodedMsg, null, 2),
      ' from ',
      rinfo
    );
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
