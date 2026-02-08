# Lesson 13: Testing

This lesson covers writing and running tests in Rust.

## Concepts Covered

- Unit tests with `#[test]`
- Assertions: `assert_eq!`, `assert!`
- Testing `Result` and `Option`
- Panic tests with `#[should_panic]`

## Running the Lesson

1. Navigate to the `lesson13` directory:

   ```bash
   cd lesson13
   ```

2. Run the script:
   ```bash
   ./run_lesson13.sh
   ```

## Expected Output

```
running 4 tests
test tests::test_add ... ok
test tests::test_divide ... ok
test tests::test_panic ... ok
test tests::test_result ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Compiling lesson13 v0.1.0 (/Users/bbx/github/rust-tutorial/lesson13)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/lesson13`
Run 'cargo test' to execute the tests
Add result: 8
```

## Key Takeaways

- Tests are marked with `#[test]`
- Use `assert_eq!` for equality checks
- `#[cfg(test)]` includes test code only during testing
- `cargo test` runs all tests
- Tests help ensure code correctness
