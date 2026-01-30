import dgram, { Socket, type RemoteInfo } from 'node:dgram';
import type { AddressInfo } from 'node:net';
import type { ConfigManager } from '../../config/ConfigManager.js';
import { IllegalAddressError } from '../../errors/result/IllegalAddressError.js';
import type { Logger } from '../../logging/Logger.js';
import { Result, type TResult } from '../../result/Result.js';
import { Module } from '../Module.js';
import { IP_ADDRESS_REGEX } from './TransportLayerSubsystem.js';
import { WireProtocolModule } from './wire-protocol/WireProtocolModule.js';

export class UDPSocket extends Module {
  private readonly socket: Socket;
  private readonly wireProtocolModule: WireProtocolModule;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.socket = dgram.createSocket({ type: 'udp4' });

    this.socket.on('error', (error) => {
      this.onError(error);
    });
    this.socket.on('close', () => {
      this.onClose();
    });
    this.socket.on('listening', () => {
      this.onListening();
    });
    this.socket.on('message', (msg, rinfo) => {
      this.onMessage(msg, rinfo);
    });

    this.wireProtocolModule = new WireProtocolModule(logger, config);
  }

  /**
   * Send a message as buffer to a specified address and port.
   * @param msg Message to be sent as buffer.
   * @param address The address to send the message to.
   * @param port The port to send the message to.
   * @returns {@link TResult<void, IllegalAddressError>}
   */
  public sendIPv4(msg: Buffer, address: string, port: number): TResult<void, IllegalAddressError> {
    if (!IP_ADDRESS_REGEX.test(address)) {
      return Result.fail(new IllegalAddressError(`${address} is not a valid IPv4 address.`));
    }

    if (port < 0 || port > 65535) {
      return Result.fail(new IllegalAddressError(`Illegal port received: ${port.toString()}. Must be between 0-65535`));
    }

    this.socket.send(msg, 0, msg.length, port, address);
    return Result.ok(undefined);
  }

  public bind(): void {
    const address = this.config.getConfig().transportLayerSubsystem.downstreamModule.dnsIPAddress;
    this.socket.bind(53, address);
    this.logger.info('Bound to ', address, ':53');
  }

  public close(): void {
    this.socket.close();
  }

  public getAddressInfo(): AddressInfo {
    return this.socket.address();
  }

  private onMessage(msg: Buffer, rinfo: RemoteInfo): void {
    const decodedMsg = this.wireProtocolModule.decode(msg);
    this.logger.debug('Received msg: ', decodedMsg, ' from ', rinfo);
  }

  private onListening(): void {
    const address = this.socket.address();
    this.logger.info(`Server listening on ${address.address}:${address.port.toString()}.`);
  }

  private onClose(): void {
    this.logger.info('Server closed.');
  }

  private onError(error: Error): void {
    this.logger.error('Server error: ', error);
  }
}
