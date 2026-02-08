# Lesson 5: Error Handling

This lesson covers error handling in Rust using `Result`, `Option`, custom error types, and the `?` operator.

## Concepts Covered

- `Option<T>`: Represents optional values (Some or None)
- `Result<T, E>`: Represents success (Ok) or failure (Err)
- Custom error types with `enum`
- Error propagation with the `?` operator
- Converting errors with `From` trait

## Running the Lesson

1. Navigate to the `lesson5` directory:

   ```bash
   cd lesson5
   ```

2. Run the script:
   ```bash
   ./run_lesson5.sh
   ```

## Expected Output

```
Some number: Some(42)
None number: None
10 / 2 = 5
Error: Division by zero
Error reading file: Io(Os { code: 2, kind: NotFound, message: "No such file or directory" })
Custom error: Custom("Something went wrong")
```

The program demonstrates:

- Using `Option` for optional values
- Using `Result` for operations that can fail
- Custom error handling with enums
- Error propagation and conversion

## Key Takeaways

- Use `Option` when a value might be absent
- Use `Result` when operations can fail
- Define custom error types for better error handling
- The `?` operator simplifies error propagation
- Implement `From` for automatic error conversion
