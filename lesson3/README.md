# Rust Tutorial - Lesson 3

Lesson 3 introduces structs for grouping related data and methods for defining behavior on structs.

## Goals

- Define custom data types with `struct`
- Implement methods with `impl` blocks
- Use `&self` for borrowing in methods
- Create associated functions (like constructors)

## Prerequisites

You should have Rust installed (see Lesson 1 for setup).

## Code

The lesson code is in [lesson3.rs](lesson3.rs).

## Run

From this folder:

```bash
rustc lesson3.rs
./lesson3
```

## What to Notice

- `struct` defines a new type with named fields.
- `impl` blocks contain methods that operate on the struct.
- `&self` borrows the struct instance immutably.
- Associated functions like `square` don't take `self` and are called with `::`.
- Methods are called with dot notation: `rect.area()`.
