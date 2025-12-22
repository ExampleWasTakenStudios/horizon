# Logging

Horizon features its own logging system designed to be high performant and extensible.

## Logger

A logger instance exposes methods to log to the following log levels:

- `FATAL`
- `ERROR`
- `WARN`
- `INFO`
- `VERBOSE`
- `DEBUG`

Each logger instance holds a Set of transports that it forwards log events to if the transport has expressed interest in the log level. For more information see the [Transports](#transports) section below.

## Transports

As is common in modern logging solutions, the system uses transports to send logs to various places.

Transports express interest in a log level by exposing a `maxLevel` property. Any log event with a level higher than `maxLevel` is not forwarded to the transport by the logging instance.

### Creation

Transports are created by extending the [`BaseTransport.ts`](../src/logging/transports/BaseTransport.ts) class.

### Current transports are:

- [`ConsoleTransport.ts`](../src/logging/transports/ConsoleTransport.ts) - Outputs pretty logs to the console.
- [`RotatingFileTransport.ts`](../src/logging/transports/RotatingFileTransport.ts) - Outputs JSON logs to a rotating file on disk. This transport also manages the rotation.
