import { LOG_LEVEL } from '../LogLevel.js';
import { ConsoleTransport } from './ConsoleTransport.js';

describe('Console Transport', () => {
  const consoleTransport = new ConsoleTransport(LOG_LEVEL.DEBUG);

  it('should return correct log level', () => {
    expect(consoleTransport.getMaxLevel()).toBe(LOG_LEVEL.DEBUG);
  });

  it('should set correct log level', () => {
    consoleTransport.setMaxLevel(LOG_LEVEL.ERROR);

    expect(consoleTransport.getMaxLevel()).toBe(LOG_LEVEL.ERROR);
  });
});
