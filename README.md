> **Work state:** SCAFFOLD · **Progress:** `███░░░░░░░ 25%`
> Rust zero-cost structured-logging framework (logkit, hexagonal); early, 17 files · updated 2026-06-02

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
