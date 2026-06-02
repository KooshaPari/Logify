> **Work state:** SCAFFOLD · **Progress:** `██░░░░░░░░ 20%`
> Zero-cost structured-logging framework (Rust, hexagonal). 17 files, early skeleton; crate name `logkit` vs repo `Logify` drift; README below is a raw lib.rs doc-comment and needs proper prose. · updated 2026-06-02

# logkit - Zero-cost Structured Logging Framework

Hexagonal architecture with ports and adapters.

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
