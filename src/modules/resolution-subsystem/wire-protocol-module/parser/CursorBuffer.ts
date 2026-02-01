import type { int16, int32, int8, uint16, uint32, uint8 } from '@src/types/number-types.js';
import { Cursor } from './Cursor.js';

/**
 * This class wraps the normal {@link Buffer} object. It provides methods to read signed and unsigned integers ranging between 8-32 bits.
 * It keeps track of the reading position through an internal cursor that is incremented whenever the buffer is read.
 *
 * All methods are big-endian unless stated otherwise.
 */
export class CursorBuffer {
  private readonly cursor: Cursor;

  private readonly buffer: Buffer;

  public constructor(buffer: Buffer, startPosition = 0) {
    this.buffer = buffer;
    this.cursor = new Cursor(startPosition);
  }

  /**
   * Forks the `CursorBuffer` by creating a shallow copy of the buffer and a cursor set to `position`.
   * @param position The position of the cursor.
   * @returns A new `CursorBuffer`.
   */
  public fork(position: number): CursorBuffer {
    return new CursorBuffer(this.buffer, position);
  }

  /**
   * Gets the current cursor position.
   * @returns A number, indicating the current cursor position.
   */
  public getCursorPosition(): number {
    return this.cursor.getPosition();
  }

  /**
   * Returns new `Buffer` that references the same memory as the original (shallow copy), but offset and cropped by the current cursor position and `length`.
   * @param length Number of bytes that should be parsed, starting from the current cursor position.
   * @returns A new {@link Buffer<ArrayBufferLike>} containing the requested section of the original buffer.
   */
  public nextSubarray(length: number): Buffer {
    const array = this.buffer.subarray(this.cursor.getPosition(), this.cursor.getPosition() + length);
    this.cursor.advance(length);

    return array;
  }

  public getBuffer(): Buffer {
    return this.buffer;
  }

  public getRemaining(): number {
    return this.buffer.length - this.cursor.getPosition();
  }

  public readNextUint8(): uint8 {
    const uint8 = this.buffer.readUint8(this.cursor.getPosition());
    this.cursor.advance(1);

    return uint8;
  }

  public readNextUint16(): uint16 {
    const uint16 = this.buffer.readUint16BE(this.cursor.getPosition());
    this.cursor.advance(2);

    return uint16;
  }

  public readNextUint32(): uint32 {
    const uint32 = this.buffer.readUint32BE(this.cursor.getPosition());
    this.cursor.advance(4);

    return uint32;
  }

  public readNextInt8(): int8 {
    const int8 = this.buffer.readInt8(this.cursor.getPosition());
    this.cursor.advance(1);

    return int8;
  }

  public readNextInt16(): int16 {
    const int16 = this.buffer.readInt16BE(this.cursor.getPosition());
    this.cursor.advance(2);

    return int16;
  }

  public readNextInt32(): int32 {
    const int32 = this.buffer.readInt32BE(this.cursor.getPosition());
    this.cursor.advance(4);

    return int32;
  }
}
