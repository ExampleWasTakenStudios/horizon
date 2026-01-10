import type { ConfigManager } from '../../../config/ConfigManager.js';
import type { Logger } from '../../../logging/Logger.js';
import type { ReturnResult } from '../../../utils/result-pattern.js';
import { Module } from '../../Module.js';
import type { DNS_RESPONSE_CODES } from './DNS-core/constants/DNS_RESPONSE_CODES.js';
import { DNSMessage } from './DNS-core/DNSMessage.js';
import { DNSParser } from './parser/DNSParser.js';

export class WireProtocolModule extends Module {
  private readonly parser: DNSParser;

  constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.parser = new DNSParser();
  }

  decode(buffer: Buffer): ReturnResult<DNSMessage, DNS_RESPONSE_CODES> {
    return this.parser.parse(buffer);
  }

  encode(_dnsPacket: DNSMessage): Buffer {
    // TODO: implement DNS packet encoder
    return Buffer.from('');
  }
}
