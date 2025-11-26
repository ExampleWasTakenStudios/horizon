/**
 * **RULE 0: If you access this cursor, you probably need to advance it once you're done with business.**
 */
export class Cursor {
  private cursor: number;

  constructor(cursor: number = 0) {
    this.cursor = cursor;
  }

  advance(n: number): void {
    this.cursor += n;
  }

  clone(): Cursor {
    return new Cursor(this.cursor);
  }

  getPosition(): number {
    return this.cursor;
  }
}
