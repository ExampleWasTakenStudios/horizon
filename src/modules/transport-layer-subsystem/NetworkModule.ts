import dgram, { Socket, type RemoteInfo } from 'node:dgram';
import type { AddressInfo } from 'node:net';
import type { ConfigManager } from '../../config/ConfigManager.js';
import { IllegalAddressError } from '../../errors/result/IllegalAddressError.js';
import { ResultError } from '../../errors/result/ResultError.js';
import type { Logger } from '../../logging/Logger.js';
import { Result, type TResult } from '../../result/Result.js';
import { Module } from '../Module.js';

export const IPv4_ADDRESS_REGEX =
  /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

export class NetworkModule extends Module {
  private readonly udpSocket: Socket;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.udpSocket = dgram.createSocket({ type: 'udp4' });

    this.onErrorUDP4((error) => {
      this.logger.error('Error occurred on UDP4 socket. Closing Socket.\nError: ', error);
      this.closeUDP4();
    });

    this.udpSocket.on('close', () => {
      this.logger.info('Server closed.');
    });
    this.udpSocket.on('listening', () => {
      const address = this.udpSocket.address();
      this.logger.info(`Server listening on ${address.address}:${address.port.toString()}.`);
    });
  }

  /**
   * Send a message as buffer to a specified IPv4 address and port.
   * @param msg Message to be sent as buffer.
   * @param address The address to send the message to.
   * @returns `TResult<void, IllegalAddressError>`
   */
  public sendUDP4(msg: Buffer, address: string, port: number): TResult<void, IllegalAddressError> {
    if (!IPv4_ADDRESS_REGEX.test(address)) {
      return Result.fail(new IllegalAddressError(`${address} is not a valid IPv4 address.`));
    }

    if (port < 0 || port > 65535) {
      return Result.fail(new IllegalAddressError(`Illegal port received: ${port.toString()}. Must be between 0-65535`));
    }

    this.logger.debug('SENDING DATA UDP4 TO: ', address, ':', port);

    this.udpSocket.send(msg, 0, msg.length, port, address);

    return Result.ok(undefined);
  }

  /**
   * Called whenever data is received on the UDP4 socket.
   * @param callback The callback to execute when data is received.
   */
  public onReceiveUDP4(callback: (payload: Buffer, rinfo: RemoteInfo) => void): void {
    this.udpSocket.on('message', (msg, rinfo) => {
      callback(msg, rinfo);
    });
  }

  /**
   * Called whenever an error occurs. An error will ALWAYS cause the socket to be closed. This event exists for callers to detect errors and open a new socket.
   * @param callback The callback to execute when an error occurs. Gets passed the error has arg.
   */
  public onErrorUDP4(callback: (error: Error) => void): void {
    this.udpSocket.on('error', (error: Error) => {
      callback(error);
    });
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

  /**
   * Closes all sockets managed by this module.
   */
  public close(): void {
    this.udpSocket.close();
  }

  /**
   * Closes the UDP4 socket managed by this module.
   */
  public closeUDP4(): void {
    this.udpSocket.close();
  }

  public getAddressInfo(): AddressInfo {
    return this.udpSocket.address();
  }
}
