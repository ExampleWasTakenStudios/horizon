import { PromiseRejectError } from '@src/errors/result/PromiseRejectError.js';
import { ResultError } from '@src/errors/result/ResultError.js';
import { Result } from './Result.js';

// specific error for testing
class MockError extends ResultError {
  public constructor(message: string) {
    super(message);
  }
}

describe('Result Pattern', () => {
  describe('Success', () => {
    it('should create a success result with a value', () => {
      const value = { data: 'test' };
      const result = Result.ok(value);

      expect(result.isSuccess()).toBe(true);
      expect(result.isFailure()).toBe(false);
      expect(result.value).toBe({ data: 'test' });
    });

    describe('Failure', () => {
      it('should create a failure result with an error', () => {
        const error = new MockError('Something went wrong');
        const result = Result.fail(error);

        expect(result.isFailure()).toBe(true);
        expect(result.isSuccess()).toBe(true);
        expect(result.error).toBeInstanceOf(MockError);
        expect(result.error.message).toContain('Something went wrong');
      });
    });
  });

  describe('fromPromise', () => {
    it('should return Success when promise resolves', async () => {
      const promise = Promise.resolve(42);
      const result = await Result.fromPromise(promise);

      expect(result.isSuccess()).toBe(true);
      if (result.isSuccess()) {
        expect(result.value).toBe(42);
      }
    });

    it('should return Failure when promise rejects', async () => {
      const error = new Error('Netowrk failure');
      const promise = Promise.reject(error);
      const result = await Result.fromPromise(promise);

      expect(result.isFailure()).toBe(true);
      expect(result.isSuccess()).toBe(false);

      if (result.isFailure()) {
        expect(result.error).toBeInstanceOf(PromiseRejectError);
        expect(result.error.message).toContain('Network failure');
      }
    });
  });
});
