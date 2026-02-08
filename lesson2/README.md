# Rust Tutorial - Lesson 2

Lesson 2 introduces ownership and borrowing with `String` and `&str`, plus a few common helper patterns like slices.

## Goals

- Understand the difference between `String` and `&str`
- See how ownership moves values between functions
- Use immutable borrows (`&T`) to read data without moving it
- Work with a string slice returned from a helper

## Prerequisites

You should have Rust installed (see Lesson 1 for setup).

## Code

The lesson code is in [lesson2.rs](lesson2.rs).

## Run

From this folder:

```bash
rustc lesson2.rs
./lesson2
```

## What to Notice

- Passing `&name` borrows the `String` without moving it.
- Passing `name` to `add_tag` moves ownership into the function.
- `clone()` explicitly makes a deep copy of heap data.
- `first_word` returns a slice tied to the input string.
