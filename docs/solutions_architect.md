# Solutions Architect Notes

This document captures design considerations and trade‑offs for the order book project.

## Technology Choices
- **Node.js** remains the public API layer for ease of use and ecosystem support.
- **Rust** implements the core matching engine to maximize performance and memory safety.
- Communication between Node.js and Rust is handled via the [N-API](https://nodejs.org/api/n-api.html) through the `napi-rs` crate.

## Deployment Strategy
1. Build the Rust library in release mode.
2. Publish the resulting Node.js package which contains the prebuilt native module.
3. Consumers install via npm/yarn without needing to compile Rust themselves.

## Scalability
- The order book state is kept in memory for speed and is persisted to disk periodically using snapshots.
- Horizontal scaling can be achieved by sharding instruments across separate instances of the service.

## Security
- Input parameters are validated before reaching the Rust layer to prevent undefined behavior.
- Continuous integration enforces linting, formatting and exhaustive testing in both TypeScript and Rust code.

