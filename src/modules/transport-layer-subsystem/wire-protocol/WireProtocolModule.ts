import type { ConfigManager } from '../../../config/ConfigManager.js';
import type { DNSParseError } from '../../../errors/result/DNSParseError.js';
import type { Logger } from '../../../logging/Logger.js';
import type { TResult } from '../../../result/Result.js';
import { Module } from '../../Module.js';
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
}
