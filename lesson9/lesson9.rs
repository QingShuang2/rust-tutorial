fn main() {
    // Closures
    let add_one = |x: i32| x + 1;
    println!("1 + 1 = {}", add_one(1));

    let add_two = |x| x + 2; // Type inferred
    println!("1 + 2 = {}", add_two(1));

    let add_three = |x: i32| -> i32 { x + 3 };
    println!("1 + 3 = {}", add_three(1));

    // Closures capturing environment
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("y == x: {}", equal_to_x(y));

    // Fn, FnMut, FnOnce
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Iterator adaptors
    let v2: Vec<i32> = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
    println!("Mapped: {:?}", v3);

    // Filter
    let v4: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let v5: Vec<i32> = v4.into_iter().filter(|x| x % 2 == 0).collect();
    println!("Filtered: {:?}", v5);

    // Custom iterator
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();

    println!("Counter values:");
    for num in counter {
        println!("{}", num);
    }

    // Using closures with iterators
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("Shoes in my size: {:?}", in_my_size);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}