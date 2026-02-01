# Error Handling

This document outlines the architectural philosophy and practical rules for handling errors within this project.

# 1. Resilience Philosophy

**The application must prioritize availability during runtime but prioritize correctness during startup.**

- **Runtime (Request Lifecycle):** The server must never crash due to a transient error processing a single packet. Any error occurring during the handling of a DNS query must be caught, logged and converted into an appropriate DNS Response Code (RCODE).
- **Startup & Configuration:** Errors occurring during initialization (binding ports, loading zone files, loading config files) must cause an immediate, controlled crash. The application should not start in an undefined or insecure state.

**Rationale**: As critical network infrastructure, Horizon acts as a foundational dependency for other services. Silent failures during startup are dangerous, but runtime crashes causes network-wide outages.

# 2. Control Flow: The Result Pattern

Avoid throwing Exceptions for control flow. Instead, use a functional **Result Pattern**.

- **Usage:** All functions that can fail must return a `TResult<T, E extends ResultError>`.
- **Implementation:** The implementation of the result pattern can be found at
  [src/result/Result.ts](../../src/result/Result.ts).
- **Checking:** Callers are forces to check the status of the Result before accessing the data.

**Rationale**: TypeScript Exceptions are opaque and easy to miss in signatures. The Result pattern forces developers to acknowledge and handle potential failures explicitly, leading to cleaner, more maintainable and safer code.

# 3. Error Propagation & Context (Bubbling)

Errors should bubble up the call stack, but they must not arrive naked.

- **Do Not Swallow Error:** Never catch an error just to return a replacement value such as `null` or `false` unless explicitly designed as a boolean check.
- **Wrap, Don't Just Return:** When an error bubbles up from a low-level method, the intermediate layer should attach context before passing it on.
- _Bad:_ receiving a "File not found" error and passing it up unchanged.
- _Good:_ receiving a "File not found" error and wrapping it in a "Failed to load zone file for example.com" error.

**Rationale**: A low-level "index out of bounds" error is useless to the operator. An "index out of bounds while parsing Resource Record in a Query" is actionable.

> Layers receiving an error may elect to not wrap it in cases where the origin of the error has enough context to create a meaningful error message.

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
