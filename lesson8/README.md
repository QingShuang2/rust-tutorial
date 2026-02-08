# Lesson 8: Lifetimes

This lesson covers lifetimes in Rust, which ensure references are valid for the duration they're used.

## Concepts Covered

- Lifetime annotations syntax
- Lifetime parameters in functions
- Lifetime parameters in structs
- Lifetime elision rules
- Static lifetime

## Running the Lesson

1. Navigate to the `lesson8` directory:

   ```bash
   cd lesson8
   ```

2. Run the script:
   ```bash
   ./run_lesson8.sh
   ```

## Expected Output

```
The longest string is abcd
Important excerpt: ImportantExcerpt { part: "Call me Ishmael" }
First word: hello
Static string: I have a static lifetime.
```

## Key Takeaways

- Lifetimes ensure references don't outlive the data they point to
- Lifetime annotations tell Rust how generic lifetime parameters relate
- The compiler often infers lifetimes with elision rules
- `'static` lifetime lasts for the entire program duration
