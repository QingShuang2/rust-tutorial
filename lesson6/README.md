# Lesson 6: Generics and Traits

This lesson covers generics and traits in Rust, allowing for code reusability and abstraction.

## Concepts Covered

- Generic functions and structs
- Trait definitions and implementations
- Trait bounds
- Using traits as function parameters

## Running the Lesson

1. Navigate to the `lesson6` directory:

   ```bash
   cd lesson6
   ```

2. Run the script:
   ```bash
   ./run_lesson6.sh
   ```

## Expected Output

```
The largest number is 100
The largest char is y
Integer point: Point { x: 5, y: 10 }
Float point: Point { x: 1.0, y: 4.0 }
Mixed point: Point { x: 5, y: 4.0 }
Mixed up point: Point { x: 5, y: 'c' }
Article summary: Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)
Tweet summary: horse_ebooks: of course, as you probably already know, people
Breaking news! Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)
Breaking news! horse_ebooks: of course, as you probably already know, people
Generic breaking news! Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)
Generic breaking news! horse_ebooks: of course, as you probably already know, people
```

## Key Takeaways

- Generics allow writing code that works with multiple types
- Traits define shared behavior that types can implement
- Use trait bounds to specify that a generic type must implement certain traits
- Traits enable polymorphism in Rust
