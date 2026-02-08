# Lesson 11: Concurrency with Threads

This lesson covers concurrency in Rust using threads, message passing, and shared state.

## Concepts Covered

- Creating and joining threads
- Moving data between threads
- Message passing with channels
- Multiple producers with channels
- Shared state with `Mutex<T>`
- `Arc<T>` for atomic reference counting

## Running the Lesson

1. Navigate to the `lesson11` directory:

   ```bash
   cd lesson11
   ```

2. Run the script:
   ```bash
   ./run_lesson11.sh
   ```

## Expected Output

The program demonstrates thread creation, message passing between threads, and shared mutable state using mutexes.

## Key Takeaways

- Threads allow concurrent execution
- Use `move` to transfer ownership to threads
- Channels enable safe communication between threads
- `Mutex<T>` provides exclusive access to shared data
- `Arc<T>` allows multiple threads to own the same data
