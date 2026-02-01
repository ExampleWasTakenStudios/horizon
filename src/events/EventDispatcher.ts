import type { EventListener, EventSource } from './EventSource.js';

export class EventDispatcher<T> implements EventSource<T> {
  private readonly listeners: EventListener<T>[];

  public constructor() {
    this.listeners = [];
  }

  public subscribe(listener: EventListener<T>): void {
    this.listeners.push(listener);
  }

  public dispatch(data: T): void {
    for (const listener of this.listeners) {
      listener(data);
    }
  }
}
