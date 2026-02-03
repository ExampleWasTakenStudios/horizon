import { ResultError } from './ResultError.js';

describe('Result Error', () => {
  // Create a nested error structure.
  const error_1 = new ResultError('Error 1');
  const error_2 = new ResultError('Error 2', error_1);
  const error_3 = new ResultError('Error 3', error_2);

  it('should return the correctly return the original error', () => {
    expect(error_3.getOriginalError()).toBe(error_1);
  });

  it('should correct chain all errors into an array', () => {
    expect(error_3.getChain()).toStrictEqual([error_3, error_2, error_1]);
  });
});
