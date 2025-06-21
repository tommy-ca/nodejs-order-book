# Product Requirements Document (PRD)

## Purpose
This document outlines the business requirements for the order book project. It serves as a guide for developers and stakeholders when building new features or improving the application.

## Goals
- Provide a high‑performance order book suitable for high frequency trading.
- Offer a simple API that can be consumed from Node.js.
- Maintain data integrity while processing large volumes of orders.

## Non‑Goals
- Building a trading user interface.
- Exchange integration or market making logic.

## Success Metrics
- Latency per matched order below 50µs when built in release mode.
- Throughput exceeding 300k matched trades per second in benchmarks.
- All unit tests passing.

## User Stories
1. **Trader** – As a trader I want to place and cancel limit or market orders so that I can trade efficiently.
2. **System Operator** – As an operator I want to persist the order book state so that the service can be recovered quickly after a restart.
3. **Developer** – As a developer I want to use a stable API so that integrations remain compatible across versions.

