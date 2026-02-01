import { ConfigManager } from '@src/config/ConfigManager.js';
import { DNSParseError } from '@src/errors/result/DNSParseError.js';
import { Logger } from '@src/logging/Logger.js';
import { Module } from '@src/modules/Module.js';
import type { TResult } from '@src/result/Result.js';
import { DNSMessage } from './DNS-core/DNSMessage.js';
import { DNSParser } from './parser/DNSParser.js';

export class WireProtocolModule extends Module {
  private readonly parser: DNSParser;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.parser = new DNSParser();
  }

  public decode(buffer: Buffer): TResult<DNSMessage, DNSParseError> {
    return this.parser.parse(buffer);
  }

  public encode(_dnsPacket: DNSMessage): Buffer {
    // TODO: implement DNS packet encoder
    return Buffer.from('');
  }

  public start(): void {
    return;
  }

  public stop(): void {
    return;
  }
}
