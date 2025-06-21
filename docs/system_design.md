# System Design Overview

The order book service exposes a Node.js interface while delegating performance‑critical logic to a Rust core library.

## Components
1. **API Layer (Node.js)** – Parses user requests, performs validation and forwards commands to the Rust core.
2. **Matching Engine (Rust)** – Maintains in‑memory data structures for bids and asks, processes new orders and matches trades using price‑time priority.
3. **Persistence Module** – Periodically snapshots the order book state to disk and applies journaled events during startup.
4. **Benchmarking Tools** – Both JavaScript and Rust benches measure throughput and latency across builds.

## Data Flow
```
Client -> Node.js API -> Rust core -> Update book -> Persist snapshot
```

Each incoming order is validated by the Node.js layer then passed to the Rust library through N‑API bindings. The Rust engine updates the book and returns matches back to Node.js, which emits events to consumers.

## Fault Tolerance
- Snapshots combined with event journals allow the service to recover to the latest confirmed state.
- Input validation and exhaustive testing reduce the likelihood of corrupted order books.

## Extensibility
- Additional order types can be added to the Rust library with minimal changes to the Node.js binding layer.
- The snapshot format is versioned to allow schema evolution.

