import { Cursor } from './Cursor.js';

describe('Cursor', () => {
  it('should advance cursor by one', () => {
    const cursor = new Cursor(0);

    // Advance cursor by one
    cursor.advance(1);
    expect(cursor.getPosition()).toBe(1);

    // Advance cursor by 5
    cursor.advance(5);
    expect(cursor.getPosition()).toBe(6);

    // Clone cursor
    const clonedCursor = cursor.clone();
    expect(clonedCursor).toBeInstanceOf(Cursor);

    // Expect the cursor of the cloned cursor to be ad the same position as the
    // cursor of the original cursor class.
    expect(clonedCursor.getPosition()).toBe(6);
  });
});
