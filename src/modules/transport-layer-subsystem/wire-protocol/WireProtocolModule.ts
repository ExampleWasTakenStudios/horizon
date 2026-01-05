import type { ConfigManager } from '../../../config/ConfigManager.js';
import type { Logger } from '../../../logging/Logger.js';
import { Module } from '../../Module.js';
import { DNSPacket } from './DNS-core/DNSPacket.js';
import { CursorBuffer } from './parser/CursorBuffer.js';
import { DNSParser } from './parser/DNSParser.js';

export class WireProtocolModule extends Module {
  private readonly parser: DNSParser;

  constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.parser = new DNSParser();
  }

  decode(buffer: Buffer): DNSPacket {
    const rawPacketCursorBuffer = new CursorBuffer(buffer);

    return this.parser.parse(rawPacketCursorBuffer);
  }

  encode(_dnsPacket: DNSPacket): Buffer {
    // TODO: implement DNS packet encoder
    return Buffer.from('');
  }
}
