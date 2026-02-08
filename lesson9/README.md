# Lesson 9: Closures and Iterators

This lesson covers closures (anonymous functions) and iterators in Rust.

## Concepts Covered

- Closure syntax and types
- Capturing environment variables
- Iterator trait and methods
- Iterator adaptors (map, filter, collect)
- Implementing custom iterators

## Running the Lesson

1. Navigate to the `lesson9` directory:

   ```bash
   cd lesson9
   ```

2. Run the script:
   ```bash
   ./run_lesson9.sh
   ```

## Expected Output

```
1 + 1 = 2
1 + 2 = 3
1 + 3 = 4
y == x: true
Got: 1
Got: 2
Got: 3
Mapped: [2, 3, 4]
Filtered: [2, 4, 6]
Counter values:
1
2
3
4
5
Shoes in my size: [Shoe { size: 10, style: "sneaker" }, Shoe { size: 10, style: "boot" }]
```

## Key Takeaways

- Closures are anonymous functions that can capture their environment
- Iterators provide a way to process sequences lazily
- Iterator adaptors transform iterators into new iterators
- `collect()` consumes the iterator and collects results into a collection
- Custom types can implement `Iterator` for iteration
