import path from 'path';
import { LOG_LEVEL } from '../LogLevel.js';
import { RotatingFileTransport } from './RotatingFileTransport.js';

describe('Console Transport', () => {
  const consoleTransport = new RotatingFileTransport(
    'test-log.txt',
    {
      interval: '1d',
      // This format is required by the library
      // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
      size: `${100 * 1024 * 1024}B`, // 100 Mib
      maxFiles: 14,
      intervalUTC: true,
      history: 'rotation-history.txt',
      path: path.join(process.cwd(), 'log'), // TODO: this should eventually be configurable
      compress: false, // TODO: this should eventually be changed to check if the current env is dev or prod -> if prod compress
      omitExtension: false,
    },
    LOG_LEVEL.DEBUG
  );

  it('should return correct log level', () => {
    expect(consoleTransport.getMaxLevel()).toBe(LOG_LEVEL.DEBUG);
  });

  it('should set correct log level', () => {
    consoleTransport.setMaxLevel(LOG_LEVEL.ERROR);

    expect(consoleTransport.getMaxLevel()).toBe(LOG_LEVEL.ERROR);
  });
});
