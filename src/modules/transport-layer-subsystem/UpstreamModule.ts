import dgram from 'node:dgram';
import type { ConfigManager } from '../../config/ConfigManager.js';
import { EventDispatcher } from '../../events/EventDispatcher.js';
import type { EventListener, EventSource } from '../../events/EventSource.js';
import type { Logger } from '../../logging/Logger.js';
import { Module } from '../Module.js';
import type { NetworkModule } from './NetworkModule.js';
import type { ReceivedData } from './ReceivedData.js';

export class UpstreamModule extends Module implements NetworkModule, EventSource<ReceivedData> {
  private readonly socket: dgram.Socket;

  private readonly dispatcher: EventDispatcher<ReceivedData>;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.socket = dgram.createSocket({ type: 'udp4' });

    this.socket.on('listening', () => {
      this.logger.verbose('Listening...');
    });

    this.socket.on('error', (error) => {
      this.logger.error('Error occurred on socket. Closing.\nError: ', error);
      this.socket.close();
    });

    this.dispatcher = new EventDispatcher();
  }

  public send(data: Buffer, address: string, port: number): void {
    this.logger.debug('Sending data to: ', address, ':', port);
    this.socket.send(data, port, address);
  }

  public subscribe(listener: EventListener<ReceivedData>): void {
    this.dispatcher.subscribe(listener);
  }

  public start(): void {
    this.socket.on('message', (msg, rinfo) => {
      this.logger.debug('Received data from: ', rinfo);
      this.dispatcher.dispatch({ buf: msg, rinfo });
    });
  }

  public stop(): void {
    this.logger.verbose('Stopping...');

    this.logger.verbose('Closing socket...');
    this.socket.close();

    this.logger.info('Stopped.');
  }
}
