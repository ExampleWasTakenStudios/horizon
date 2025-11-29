import { Cursor } from './Cursor.js';

/**
 * This class wraps the normal {@link Buffer} object. It provides methods to read signed and unsigned integers ranging between 8-32 bits.
 * It keeps track of the reading position through an internal cursor that is incremented whenever the buffer is read.
 *
 * All methods are big-endian unless stated otherwise.
 */
export class CursorBuffer {
  private cursor: Cursor;

  private buffer: Buffer;

  constructor(buffer: Buffer, startPosition: number = 0) {
    this.buffer = buffer;
    this.cursor = new Cursor(startPosition);
  }

  /**
   * Clones this instance to a new, indpendent `CursorBuffer`.
   * @returns A new, independent `CursorBuffer`.
   */
  clone(): CursorBuffer {
    return new CursorBuffer(Buffer.from(this.buffer), this.cursor.getPosition());
  }

  /**
   * Gets the current cursor position.
   * @returns A number, indicating the current cursor position.
   */
  getCursorPosition(): number {
    return this.cursor.getPosition();
  }

  /**
   * Allocates a new {@link Buffer<ArrayBufferLike>} and returns it.
   * @param length Number of bytes that should be parsed, starting from the current cursor position.
   * @returns A new {@link Buffer<ArrayBufferLike>} containing the requested section of the original buffer.
   */
  subarray(length: number): Buffer<ArrayBufferLike> {
    const array = Buffer.from(this.buffer.subarray(this.cursor.getPosition()));
    this.cursor.advance(length);

    return array;
  }

  /**
   * Creates a clone of the buffer of the cursor, by allocating a new buffer from the buffer of this instance.
   */
  cloneBuffer(): Buffer {
    return Buffer.from(this.buffer);
  }

  readUint8(): number {
    const u_int8 = this.buffer.readUint8(this.cursor.getPosition());
    this.cursor.advance(1);

    return u_int8;
  }

  readUint16(): number {
    const u_int16 = this.buffer.readUint16BE(this.cursor.getPosition());
    this.cursor.advance(2);

    return u_int16;
  }

  readUint32(): number {
    const u_int32 = this.buffer.readUint32BE(this.cursor.getPosition());
    this.cursor.advance(4);

    return u_int32;
  }

  readInt8(): number {
    const int8 = this.buffer.readInt8(this.cursor.getPosition());
    this.cursor.advance(1);

    return int8;
  }

  readInt16(): number {
    const int16 = this.buffer.readInt16BE(this.cursor.getPosition());
    this.cursor.advance(2);

    return int16;
  }

  readInt32(): number {
    const int32 = this.buffer.readInt32BE(this.cursor.getPosition());
    this.cursor.advance(4);

    return int32;
  }

  /**
   * Retrieve a value from a buffer at a requested location. This method DOES NOT advance the internal cursor and is only meant for lookups, not parsing.
   * @param cursor A cursor object pointing at the location in the buffer.
   * @returns The value at the requested position.
   */
  peekUint8At(cursor: Cursor): number {
    return this.buffer.readUint8(cursor.getPosition());
  }

  /**
   * Retrieve a value from a buffer at a requested location. This method DOES NOT advance the internal cursor and is only meant for lookups, not parsing.
   * @param cursor A cursor object pointing at the location in the buffer.
   * @returns The value at the requested position.
   */
  peekUint16At(cursor: Cursor): number {
    return this.buffer.readUInt16BE(cursor.getPosition());
  }

  /**
   * Retrieve a value from a buffer at a requested location. This method DOES NOT advance the internal cursor and is only meant for lookups, not parsing.
   * @param cursor A cursor object pointing at the location in the buffer.
   * @returns The value at the requested position.
   */
  peekUint32At(cursor: Cursor): number {
    return this.buffer.readUint32BE(cursor.getPosition());
  }

  /**
   * Retrieve a value from a buffer at a requested location. This method DOES NOT advance the internal cursor and is only meant for lookups, not parsing.
   * @param cursor A cursor object pointing at the location in the buffer.
   * @returns The value at the requested position.
   */
  peekInt8At(cursor: Cursor): number {
    return this.buffer.readInt8(cursor.getPosition());
  }

  /**
   * Retrieve a value from a buffer at a requested location. This method DOES NOT advance the internal cursor and is only meant for lookups, not parsing.
   * @param cursor A cursor object pointing at the location in the buffer.
   * @returns The value at the requested position.
   */
  peekInt16At(cursor: Cursor): number {
    return this.buffer.readInt16BE(cursor.getPosition());
  }

  /**
   * Retrieve a value from a buffer at a requested location. This method DOES NOT advance the internal cursor and is only meant for lookups, not parsing.
   * @param cursor A cursor object pointing at the location in the buffer.
   * @returns The value at the requested position.
   */
  peekInt32At(cursor: Cursor): number {
    return this.buffer.readInt32BE(cursor.getPosition());
  }
}
