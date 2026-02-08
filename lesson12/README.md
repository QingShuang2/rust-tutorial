# Lesson 12: Async Programming

This lesson introduces asynchronous programming in Rust using Tokio.

## Concepts Covered

- Async functions and `.await`
- Concurrent tasks with `tokio::spawn`
- Futures and `Pin<Box<dyn Future>>`
- Tokio runtime

## Prerequisites

This lesson uses Cargo for dependency management. Make sure Cargo is installed.

## Running the Lesson

1. Navigate to the `lesson12` directory:

   ```bash
   cd lesson12
   ```

2. Run the script:
   ```bash
   ./run_lesson12.sh
   ```

## Expected Output

```
Starting async lesson
Hello from async function!
5 + 3 = 8
Task 1 starting
Task 2 starting
Task 2 finished
Task 1 finished
Results: 42 and 24
Future result: 100
```

## Key Takeaways

- `async fn` defines asynchronous functions
- `.await` waits for futures to complete
- `tokio::spawn` runs tasks concurrently
- Futures represent values that may not be ready yet
- Tokio provides the runtime for executing async code
