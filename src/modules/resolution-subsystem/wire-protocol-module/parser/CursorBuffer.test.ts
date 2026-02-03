import { CursorBuffer } from './CursorBuffer.js';

describe('CursorBuffer', () => {
  it('should read next uint8 from buffer and advance cursor', () => {
    const buffer = Buffer.from([0x05, 0x0a]);
    const cursorBuffer = new CursorBuffer(buffer);

    expect(cursorBuffer.readNextUint8()).toBe(0x05);
    expect(cursorBuffer.getCursorPosition()).toBe(1);
    expect(cursorBuffer.readNextUint8()).toBe(0x0a);
    expect(cursorBuffer.getCursorPosition()).toBe(2);
  });

  it('should read next uint16 from buffer and advance cursor', () => {
    const buffer = Buffer.from([0x01, 0x0a]);
    const cursorBuffer = new CursorBuffer(buffer);

    expect(cursorBuffer.readNextUint16()).toBe(266);
    expect(cursorBuffer.getCursorPosition()).toBe(2);
  });

  it('should extract a subarray from the buffer and advance cursor', () => {
    const buffer = Buffer.from([0xa1, 0x11, 0x34]);
    const cursorBuffer = new CursorBuffer(buffer);

    expect(cursorBuffer.nextSubarray(3)).toStrictEqual(Buffer.from([0xa1, 0x11, 0x34]));
    expect(cursorBuffer.getCursorPosition()).toBe(3);
  });

  it('should correctly report remaining bytes', () => {
    const buffer = Buffer.from([0x34, 0x24, 0x54, 0xab]);
    const cursorBuffer = new CursorBuffer(buffer);

    expect(cursorBuffer.getRemaining()).toBe(4);

    // Read one byte
    cursorBuffer.readNextUint8();
    expect(cursorBuffer.getRemaining()).toBe(3);

    // Read two bytes
    cursorBuffer.readNextUint16();
    expect(cursorBuffer.getRemaining()).toBe(1);
  });

  describe('fork', () => {
    it('should fork the CursorBuffer with a shallow copy of the buffer', () => {
      const buffer = Buffer.from([0xab, 0x2d, 0x23]);
      const cursorBuffer = new CursorBuffer(buffer);

      const forkedBuffer = cursorBuffer.fork(0);
      expect(forkedBuffer.getBuffer()).toBe(buffer);
    });

    it('should set the cursor to the correct position on the forked buffer', () => {
      const buffer = Buffer.from([0xab, 0x2d, 0x23]);
      const cursorBuffer = new CursorBuffer(buffer);

      const forkedBuffer = cursorBuffer.fork(2);
      expect(forkedBuffer.getCursorPosition()).toBe(2);
    });
  });

  describe('Boundary Checks', () => {
    it('should throw when reading past end of buffer', () => {
      const buffer = Buffer.from([0x33, 0x21]);
      const cursorBuffer = new CursorBuffer(buffer);

      cursorBuffer.readNextUint16();

      expect(() => cursorBuffer.readNextUint8()).toThrow();
    });
  });
});
