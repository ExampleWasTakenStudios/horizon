import { Cursor } from './Cursor.js';

export class CursorBuffer {
  private cursor: Cursor;

  private buffer: Buffer;

  constructor(buffer: Buffer) {
    this.buffer = buffer;
    this.cursor = new Cursor();
  }
}
