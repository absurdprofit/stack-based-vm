# Stack-Based Virtual Machine — Rust Implementation

A Rust port of a [Tsoding's simle stack-based virtual machine](https://gist.github.com/rexim/a52f89e6500ac6328f017d0db1b518b8) originally written in C. This implementation executes a sequence of instructions (Push, Add, Print) over a stack and demonstrates enum-based static dispatch using `enum_dispatch`.

---

## Overview

The VM operates on a simple instruction set:

* `Push(i32)` — pushes a value onto the stack
* `Add` — pops two values, adds them, and pushes the result
* `Print` — pops and prints the top value

Execution is performed sequentially over a program represented as a vector of instructions.

---

## Example Program

```rust
let program: Vec<InstructionSet> = vec![
    Push::new(50).into(),
    Push::new(51).into(),
    Add.into(),
    Print.into(),
];
```

Output:

```
101
```

---

## Features

* Stack-based execution model
* Enum-driven static dispatch via `enum_dispatch`
* Zero-cost abstraction (no dynamic dispatch)
* Minimal instruction set (Push, Add, Print)

---

## Requirements

* Rust toolchain (`rustup`)

---

## Build & Run

Build in release mode:

```bash
cargo build --release
```

Run:

```bash
./target/release/stack-based-vm
```

Or run directly with Cargo:

```bash
cargo run --release
```
