# Lesson 10: Smart Pointers

This lesson covers smart pointers in Rust: `Box<T>`, `Rc<T>`, `Arc<T>`, and `RefCell<T>`.

## Concepts Covered

- `Box<T>`: Heap allocation and recursive types
- `Rc<T>`: Reference counting for multiple ownership
- `RefCell<T>`: Interior mutability
- `Arc<T>`: Atomic reference counting for concurrency

## Running the Lesson

1. Navigate to the `lesson10` directory:

   ```bash
   cd lesson10
   ```

2. Run the script:
   ```bash
   ./run_lesson10.sh
   ```

## Expected Output

```
Box value: 5
List: Cons(1, Cons(2, Cons(3, Nil)))
Reference count after creating a: 1
Reference count after creating b: 2
Reference count after creating c: 3
Reference count after c goes out of scope: 2
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
Result: 10
```

## Key Takeaways

- Smart pointers provide additional capabilities beyond references
- `Box<T>` allocates data on the heap
- `Rc<T>` enables multiple ownership with reference counting
- `RefCell<T>` allows mutable borrowing at runtime
- `Arc<T>` is the thread-safe version of `Rc<T>`
