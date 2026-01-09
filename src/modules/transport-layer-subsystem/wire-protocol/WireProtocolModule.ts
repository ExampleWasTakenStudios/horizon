import type { ConfigManager } from '../../../config/ConfigManager.js';
import type { Logger } from '../../../logging/Logger.js';
import type { ReturnResult } from '../../../types/ReturnResult.js';
import { Module } from '../../Module.js';
import { DNSMessage } from './DNS-core/DNSMessage.js';
import { DNSParser } from './parser/DNSParser.js';

export class WireProtocolModule extends Module {
  private readonly parser: DNSParser;

  constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.parser = new DNSParser();
  }

  decode(buffer: Buffer): ReturnResult<DNSMessage> {
    return this.parser.parse(buffer);
  }

  encode(_dnsPacket: DNSMessage): Buffer {
    // TODO: implement DNS packet encoder
    return Buffer.from('');
  }
}
