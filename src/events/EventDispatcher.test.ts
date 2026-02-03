import { EventDispatcher } from './EventDispatcher.js';

class MockReceiver {
  public readonly mockDispatcher: EventDispatcher<string>;
  public data: string;

  public constructor() {
    this.mockDispatcher = new EventDispatcher();
    this.mockDispatcher.subscribe((data) => {
      this.listener(data);
    });
    this.data = '';
  }

  public listener(data: string): void {
    this.data = data;
  }
}

describe('Event Dispatcher', () => {
  const mockReceiver = new MockReceiver();

  const listenerSpy = vi.spyOn(mockReceiver, 'listener');

  it('should call the listener when an event is dispatched', () => {
    mockReceiver.mockDispatcher.dispatch('test 1');
    expect(listenerSpy).toHaveBeenCalled();
  });

  it('should pass the correct data to the listener', () => {
    mockReceiver.mockDispatcher.dispatch('test 2');
    expect(mockReceiver.data).toBe('test 2');
  });
});
