import type { ConfigManager } from '../../../config/ConfigManager.js';
import type { Logger } from '../../../logging/Logger.js';
import type { ReturnResult } from '../../../types/ReturnResult.js';
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

  decode(buffer: Buffer): ReturnResult<DNSPacket> {
    const rawPacketCursorBuffer = new CursorBuffer(buffer);

    const parseResult = this.parser.parse(rawPacketCursorBuffer);
    if (!parseResult.success) {
      return {
        success: false,
        rCode: parseResult.rCode,
      };
    }

    return {
      success: true,
      data: parseResult.data,
    };
  }

  encode(_dnsPacket: DNSPacket): Buffer {
    // TODO: implement DNS packet encoder
    return Buffer.from('');
  }
}
