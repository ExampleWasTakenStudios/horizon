import { Logger } from './Logger.js';
import { LOG_LEVEL } from './LogLevel.js';
import { ConsoleTransport } from './transports/ConsoleTransport.js';

describe('Logger', () => {
  const logger = new Logger('test_logger');
  const testTransport = new ConsoleTransport(LOG_LEVEL.ERROR);

  it('should add transport to logger', () => {
    logger.addTransport(testTransport);

    expect(logger.getTransports().has(testTransport)).toBe(true);
  });

  it('should remove transport from logger', () => {
    logger.removeTransport(testTransport);

    expect(logger.getTransports().has(testTransport)).toBe(false);
  });

  it('should correctly spawn a sub logger', () => {
    logger.addTransport(testTransport);
    const subLogger = logger.spawnSubLogger('sub_logger');

    expect(subLogger).toBeInstanceOf(Logger);
    expect(subLogger.getTransports().has(testTransport)).toBe(true);
  });
});
