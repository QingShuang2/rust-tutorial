# Rust Tutorial - Lesson 1

Lesson 1 introduces the Rust toolchain, the idea of variables and mutability, basic types, and simple control flow. It includes a short program you can compile and run.

## Goals

- Install and verify Rust tooling
- Write and run a basic Rust program
- Understand `let`, `mut`, and simple `if` logic

## Prerequisites

Install Rust with rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then verify:

```bash
rustc --version
cargo --version
```

## Code

The lesson code is in [lesson1.rs](lesson1.rs).

## Run

From this folder:

```bash
rustc lesson1.rs
./lesson1
```

## What to Notice

- `let` makes an immutable binding by default.
- Use `mut` when you want to change a value.
- `if` is an expression and can be used to pick a value.
- Rust uses `println!` (a macro) for output.
