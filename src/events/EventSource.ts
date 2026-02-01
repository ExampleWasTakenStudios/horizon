export type EventListener<T> = (data: T) => void;

export interface EventSource<T> {
  subscribe(listener: EventListener<T>): void;
}
