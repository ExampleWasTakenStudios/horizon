import type { Module } from '../Module.js';
import { DNSPacket } from './DNS-core/DNSPacket.js';
import { CursorBuffer } from './parser/CursorBuffer.js';
import { DNSParser } from './parser/DNSParser.js';

export class WireProtocolModule implements Module {
  decode(buffer: Buffer): DNSPacket {
    const parser = new DNSParser();

    const rawPacketCursorBuffer = new CursorBuffer(buffer);

    return parser.parse(rawPacketCursorBuffer);
  }

  encode(_dnsPacket: DNSPacket): Buffer {
    // TODO: implement DNS packet encoder
    return Buffer.from('');
  }
}
