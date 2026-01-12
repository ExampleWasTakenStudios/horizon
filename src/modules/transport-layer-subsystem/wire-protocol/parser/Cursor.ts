/**
 * **RULE 0: If you access this cursor, you probably need to advance it once you're done with business.**
 *
 * All values passed to this class will be rounded to the nearest integer.
 */
export class Cursor {
  private cursor: number;

  constructor(cursor = 0) {
    this.cursor = Math.round(cursor);
  }

  /**
   * Advance the cursor by {@link n} integer values.
   * @param n Amount that is added to the cursor.
   */
  advance(n: number): void {
    this.cursor += Math.round(n);
  }

  clone(): Cursor {
    return new Cursor(this.cursor);
  }

  getPosition(): number {
    return this.cursor;
  }
}
