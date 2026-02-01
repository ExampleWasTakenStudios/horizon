import { ConfigManager } from '@src/config/ConfigManager.js';
import { EventDispatcher } from '@src/events/EventDispatcher.js';
import type { EventListener, EventSource } from '@src/events/EventSource.js';
import { Logger } from '@src/logging/Logger.js';
import dgram from 'node:dgram';
import { Module } from '../Module.js';
import type { NetworkModule } from './NetworkModule.js';
import type { ReceivedData } from './ReceivedData.js';

export class DownstreamModule extends Module implements NetworkModule, EventSource<ReceivedData> {
  private readonly socket: dgram.Socket;

  private readonly dispatcher: EventDispatcher<ReceivedData>;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);
    this.socket = dgram.createSocket({ type: 'udp4' });

    this.socket.on('listening', () => {
      this.logger.info('Listening...');
    });

    this.socket.on('error', (error) => {
      this.logger.error('Error occurred on socket. Closing.\nError: ', error);
      this.socket.close();
    });

    this.dispatcher = new EventDispatcher();
  }

  public send(data: Buffer, address: string, port: number): void {
    this.logger.debug('Sending data to: ', address, ':', port);
    this.socket.send(data, port, address, (error) => {
      this.sendCallback(error);
    });
  }

  public subscribe(listener: EventListener<ReceivedData>): void {
    this.dispatcher.subscribe(listener);
  }

  public start(): void {
    this.socket.on('message', (msg, rinfo) => {
      this.logger.debug('Received data from: ', rinfo);
      this.dispatcher.dispatch({ buf: msg, rinfo });
    });

    const address = this.config.getConfig().transportLayerSubsystem.downstreamModule.dnsIPv4Address;

    this.logger.info(`Binding to ${address}:53`);
    this.socket.bind(53, address);
  }

  public stop(): void {
    this.logger.verbose('Stopping...');

    this.logger.verbose('Closing socket...');
    this.socket.close();

    this.logger.info('Stopped.');
  }

  private sendCallback(error: Error | null): void {
    if (error) {
      this.logger.error('Socket error while trying to send: ', error);
    }
  }
}
