# Rust Tutorial - Lesson 4

Lesson 4 introduces enums for defining types that can be one of several variants, and pattern matching for handling different cases.

## Goals

- Define enums with `enum`
- Use enums to create more concise data structures
- Implement methods on enums
- Use `match` for exhaustive pattern matching
- Use `if let` for concise option handling

## Prerequisites

You should have Rust installed (see Lesson 1 for setup).

## Code

The lesson code is in [lesson4.rs](lesson4.rs).

## Run

From this folder:

```bash
rustc lesson4.rs
./lesson4
```

## What to Notice

- `enum` defines a type with multiple possible variants.
- Enums can hold data directly in variants (like `IpAddr2`).
- `match` expressions handle all possible cases exhaustively.
- `if let` is a concise way to handle `Option` values.
- Methods can be implemented on enums with `impl` blocks.
