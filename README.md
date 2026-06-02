> **Work state:** SCAFFOLD · **Progress:** `███░░░░░░░ 25%`
> Zero-cost structured-logging framework (Rust, hexagonal). GOAL: the org-shared logging surface (one sink/format contract for all Phenotype Rust crates); early at 17 files. Crate name `logkit` vs repo `Logify` drift to reconcile. · updated 2026-06-02

//! # logkit - Zero-cost Structured Logging Framework
//!
//! Hexagonal architecture with ports and adapters.

## Features

- Zero-cost abstraction
- Multiple sinks
- Structured logging
- Async support

## Usage

```rust
use logkit::{LoggerBuilder, Level};

let logger = LoggerBuilder::new("app")
    .level(Level::Info)
    .build();
```
