import dgram, { Socket, type RemoteInfo } from 'node:dgram';
import type { ConfigManager } from '../../config/ConfigManager.js';
import { ResultError } from '../../errors/result/ResultError.js';
import type { Logger } from '../../logging/Logger.js';
import { Result, type TResult } from '../../result/Result.js';
import { Module } from '../Module.js';
import { WireProtocolModule } from './wire-protocol/WireProtocolModule.js';

export class UpstreamModule extends Module {
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

  public bind(): TResult<void, ResultError> {
    const address = this.config.getConfig().transportLayerSubsystem.upstreamModule.dnsIPv4Address;
    const port = this.config.getConfig().transportLayerSubsystem.upstreamModule.dnsIPv4Port;
    try {
      this.socket.bind(port, address);
    } catch (err) {
      this.logger.error('Failed to bind... ', err);
      return Result.fail(
        new ResultError(`[Downstream Module] Error while trying to bind to UDPv4 port: ${JSON.stringify(err)}`)
      );
    }
    this.logger.info('Bound to ', address, ':53');
    return Result.ok(undefined);
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
