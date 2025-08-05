# Installation

This chapter will guide you through installing and setting up FlatMessage in your Rust project.

## System Requirements

FlatMessage requires:
- **Rust**: Version 1.70 or later (2021 edition)
- **Operating System**: Cross-platform (Windows, macOS, Linux)
- **Architecture**: Support for 32-bit and 64-bit systems

## Adding FlatMessage to Your Project

The easiest way to add FlatMessage to your project is through Cargo. Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
flat_message = "*"
```

## Use it

To use FlatMessage, define a structure and derive it from `FlatMessage` like in the following example:

```rust
use flat_message::*;

#[derive(FlatMessage)]
struct TestMessage {
    // fields
}