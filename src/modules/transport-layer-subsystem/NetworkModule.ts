import dgram, { Socket, type RemoteInfo } from 'node:dgram';
import type { AddressInfo } from 'node:net';
import type { ConfigManager } from '../../config/ConfigManager.js';
import { IllegalAddressError } from '../../errors/result/IllegalAddressError.js';
import { ResultError } from '../../errors/result/ResultError.js';
import type { Logger } from '../../logging/Logger.js';
import { Result, type TResult } from '../../result/Result.js';
import { Module } from '../Module.js';
import { IPv4_ADDRESS_REGEX } from './TransportLayerSubsystem.js';
import type { WireProtocolModule } from './wire-protocol/WireProtocolModule.js';

export class NetworkModule extends Module {
  private readonly udpSocket: Socket;
  private readonly wireProtocolModule: WireProtocolModule;

  public constructor(wireProtocolModule: WireProtocolModule, logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.udpSocket = dgram.createSocket({ type: 'udp4' });

    this.udpSocket.on('error', (error) => {
      this.onError(error);
    });
    this.udpSocket.on('close', () => {
      this.onClose();
    });
    this.udpSocket.on('listening', () => {
      this.onListening();
    });
    this.udpSocket.on('message', (msg, rinfo) => {
      this.onMessage(msg, rinfo);
    });

    this.wireProtocolModule = wireProtocolModule;
  }

  public onMessage(msg: Buffer, rinfo: RemoteInfo): void {
    const decodedMsg = this.wireProtocolModule.decode(msg);
    this.logger.debug('Received msg: ', decodedMsg, ' from ', rinfo);

    // The decoded message should be sent to the request module here.
  }

  public onListening(): void {
    const address = this.udpSocket.address();
    this.logger.info(`Server listening on ${address.address}:${address.port.toString()}.`);
  }

  public onClose(): void {
    this.logger.info('Server closed.');
  }

  public onError(error: Error): void {
    this.logger.error('Server error: ', error);
  }

  /**
   * Send a message as buffer to a specified IPv4 address and port.
   * @param msg Message to be sent as buffer.
   * @param address The address to send the message to.
   * @param port The port to send the message to.
   * @returns {@link TResult<void, IllegalAddressError>}
   */
  public sendIPv4UDP(msg: Buffer, address: string, port: number): TResult<void, IllegalAddressError> {
    if (!IPv4_ADDRESS_REGEX.test(address)) {
      return Result.fail(new IllegalAddressError(`${address} is not a valid IPv4 address.`));
    }

    if (port < 0 || port > 65535) {
      return Result.fail(new IllegalAddressError(`Illegal port received: ${port.toString()}. Must be between 0-65535`));
    }

    this.udpSocket.send(msg, 0, msg.length, port, address);
    return Result.ok(undefined);
  }

  public bind(address: string, port: number): TResult<void, ResultError> {
    try {
      this.udpSocket.bind(port, address);
    } catch (err) {
      this.logger.error('Failed to bind... ', err);
      return Result.fail(
        new ResultError(`[UDP Module] Error while trying to bind to UDPv4 port: ${JSON.stringify(err)}`)
      );
    }
    this.logger.info('Bound to ', address, ':', port);
    return Result.ok(undefined);
  }

  public close(): void {
    this.udpSocket.close();
  }

  public getAddressInfo(): AddressInfo {
    return this.udpSocket.address();
  }
}
