import type { BaseTransport } from './transports/BaseTransport.js';
import type { LogEntry } from './LogEntry.js';
import { LogLevel } from './LogLevel.js';

export class Logger {
  private readonly name: string;
  private transports: Set<BaseTransport>;

  constructor(name: string, transports?: Set<BaseTransport>) {
    this.name = name;
    this.transports = transports ? transports : new Set();
  }

  private write(level: LogLevel, message: string, data?: unknown): void {
    const logEntry: LogEntry = {
      timestamp: new Date(),
      level,
      message,
      source: this.name,
      data,
    };

    for (const current of this.transports) {
      current.log(logEntry);
    }
  }

  // --- Public API ---

  fatal(message: string, data?: unknown): void {
    this.write(LogLevel.FATAL, message, data);
  }

  error(message: string, data?: unknown): void {
    this.write(LogLevel.ERROR, message, data);
  }

  warn(message: string, data?: unknown): void {
    this.write(LogLevel.WARN, message, data);
  }

  info(message: string, data?: unknown): void {
    this.write(LogLevel.INFO, message, data);
  }

  verbose(message: string, data?: unknown): void {
    this.write(LogLevel.VERBOSE, message, data);
  }

  debug(message: string, data?: unknown): void {
    this.write(LogLevel.DEBUG, message, data);
  }

  addTransport(transport: BaseTransport): void {
    this.transports.add(transport);
  }

  removeTransport(transport: BaseTransport): boolean {
    return this.transports.delete(transport);
  }

  getTransports(): Readonly<Set<BaseTransport>> {
    return this.transports;
  }

  /**
   * Get a new logger instance that is a logical child of this instance.
   * @param name The name of this sublogger. Will be appended to the name of the parent logger -> <parentName>:<name>
   * @param transports The transports that should be added to the logger. Defaults to the transports of the parent logger. If none are wanted, pass an empty Set.
   */
  getSubLogger(name: string, transports: Set<BaseTransport> = this.transports): Logger {
    return new Logger(`${this.name}:${name}`, transports);
  }
}
