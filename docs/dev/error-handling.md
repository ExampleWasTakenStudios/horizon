# Error Handling

This document outlines the architectural philosophy and practical rules for handling errors within this project.

# 1. Resilience Philosophy

**The application must prioritize availability during runtime but prioritize correctness during startup.**

- **Runtime (Request Lifecycle):** The server must never crash due to a transient error processing a single packet. Any error occurring during the handling of a DNS query must be caught, logged and converted into an appropriate DNS Response Code (RCODE).
- **Startup & Configuration:** Errors occurring during initialization (binding ports, loading zone files, loading config files) must cause an immediate, controlled crash. The application should not start in an undefined or insecure state.

**Rationale**: As critical network infrastructure, Horizon acts as a foundational dependency for other services. Silent failures during startup are dangerous, but runtime crashes causes network-wide outages.

# 2. Control Flow: The Result Pattern

Avoid throwing Exceptions for control flow. Instead, use a functional **Result Pattern**.

- **Usage:** All functions that can fail must return a `ReturnResult<T, E>`.
- **Implementation:** Use the helper methods provided in [src/utils/result-pattern.ts](../../src/utils/result-pattern.ts).
- **Checking:** Callers are forces to check the status of the Result before accessing the data.

**Rationale**: TypeScript Exceptions are opaque and easy to miss in signatures. The Result pattern forces developers to acknowledge and handle potential failures explicitly, leading to cleaner, more maintainable and safer code.

# 3. Error Propagation & Context (Bubbling)

Errors should bubble up the call stack, but they must not arrive naked.

- **Do Not Swallow Error:** Never catch an error just to return a replacement value such as `null` or `false` unless explicitly designed as a boolean check.
- **Wrap, Don't Just Return:** When an error bubbles up from a low-level method, the intermediate layer should attach context before passing it on.
- _Bad:_ receiving a "File not found" error and passing it up unchanged.
- _Good:_ receiving a "File not found" error and wrapping it in a "Failed to load zone file for example.com" error.

**Rationale**: A low-level "index out of bounds" error is useless to the operator. An "index out of bounds while parsing Resource Record in a Query" is actionable.

# 4. Logging Strategy

**Log once, log at the edge.**

- **The "Edge" Rule:** Only log errors at the "Edge" of the execution context.
- **No Double Logging:** Intermediate layers should return the `Result` without logging.

**Rationale**: This ensures that every error appears exactly once in the logs, complete with the full context chain accumulated during propagation.

# 5. DNS Protocol Specifics

Internal errors that occur during processing of a DNS Query must be mapped to the correct DNS RCODE before being sent back to the client.

| Internal Error Type  | Resulting RCODE | Description                                                                   |
| -------------------- | --------------- | ----------------------------------------------------------------------------- |
| **Parsing Error**    | `FORMERR` (1)   | The incoming packet was malformed or violated protocol spec.                  |
| **Logic/IO Error**   | `SERVFAIL` (2)  | The server failed to process a valid request (e.g. DB down, upstream timeout) |
| **Validation Error** | `NXDOMAIN` (3)  | The domain does not exist (authoritative only).                               |
| **Not Implemented**  | `NOTIMP` (4)    | The query type/opcode is not supported.                                       |
| **Policy/Auth**      | `REFUSED` (5)   | The server refuses to perform the requested action.                           |

# 6. Implementation

Below is a quick guide on how to get started with the existing error handling infrastructure.

> [!NOTE]
> The pseudo-code below is not meant to represent general coding guidelines, used in this project.

Suppose you have this structure:

```ts
/**
 * This is the low-level function interacting with the file system.
 */
const readFile = (path: string): ReturnResult<File, Error> => {
  try {
    return ok(fs.readFile(path));
  } catch (err) {
    return err({ error: err, debugInfo: ['Failed to parse file.'] })
  }
}
/**
 * This function abstracts away the low-level interaction with the file system.
 * This could be part of a larger 'ConfigManager' class that provides way of
 * interacting with the config.
 */
const getConfig = (): ReturnResult<Config, Error> => {
  const readResult = readFile('path/to/config');

  if (readResult.err) {
    return err(err.addDebugInfo('Failed to retrieve config.'));
  }

  // ...parse the retrieved data to a config object here...
  const config = readResult.data;
  return config;
}

/**
 * The business logic code would call 'getConfig()' to get a config object
 */
const doStuff = (): void => {
  const configResult = getConfig();
  if (configResult.err) {
    console.error('Something went wrong while trying to apply config: ', error, debugInfo);
    return;
  }

  // ...apply config here...
  const darkModeEnabled = configResult.data.darkModeEnabled;
}
```

Perhaps the most useful feature of this framework is the chainable `addDebugInfo()` function that allows higher layers to add their own context to the error, thus providing the developer with context about the state of the entire call structure.

E.g. in the example above, the developer not only informed that a file could not be read, but which file (`config`) and during which process (`applying the 'darkModeEnabled' property`).
