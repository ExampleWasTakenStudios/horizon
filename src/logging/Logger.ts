import type { LogEntry } from './LogEntry.js';
import { LogLevel } from './LogLevel.js';
import type { BaseTransport } from './transports/BaseTransport.js';

export class Logger {
  private readonly name: string;
  private readonly transports: Set<BaseTransport>;

  public constructor(name: string, transports?: Set<BaseTransport>) {
    this.name = name;
    this.transports = transports ?? new Set();
  }

  private write(level: LogLevel, message: string, ...data: unknown[]): void {
    let logEntry: LogEntry | undefined;

    for (const transport of this.transports) {
      if (level <= transport.getMaxLevel()) {
        logEntry ??= {
          timestamp: new Date(),
          level,
          message,
          source: this.name,
          data,
        };

        transport.log(logEntry);
      }
    }
  }

  // --- Public API ---

  public fatal(message: string, ...data: unknown[]): void {
    this.write(LogLevel.FATAL, message, ...data);
  }

  public error(message: string, ...data: unknown[]): void {
    this.write(LogLevel.ERROR, message, ...data);
  }

  public warn(message: string, ...data: unknown[]): void {
    this.write(LogLevel.WARN, message, ...data);
  }

  public info(message: string, ...data: unknown[]): void {
    this.write(LogLevel.INFO, message, ...data);
  }

  public verbose(message: string, ...data: unknown[]): void {
    this.write(LogLevel.VERBOSE, message, ...data);
  }

  public debug(message: string, ...data: unknown[]): void {
    this.write(LogLevel.DEBUG, message, ...data);
  }

  public addTransport(transport: BaseTransport): void {
    this.transports.add(transport);
  }

  public removeTransport(transport: BaseTransport): boolean {
    return this.transports.delete(transport);
  }

  public getTransports(): Readonly<Set<BaseTransport>> {
    return this.transports;
  }

  /**
   * Get a new logger instance that is a logical child of this instance.
   * @param name The name of this sublogger. Will be appended to the name of the parent logger -> <parentName>:<name>
   * @param transports The transports that should be added to the logger. Defaults to the transports of the parent logger. If none are wanted, pass an empty Set.
   */
  public spawnSubLogger(name: string, transports: Set<BaseTransport> = this.transports): Logger {
    return new Logger(`${this.name}>${name.replace(/\s/g, '_')}`, new Set(transports));
  }
}
