fn main() {
    println!("Lesson 1: Hello, Rust!");

    let name = "Learner";
    let mut score = 0;

    println!("Welcome, {name}.");

    score += 10;

    let level = if score >= 10 { "beginner" } else { "new" };

    println!("Score: {score}");
    println!("Level: {level}");

    let sum = add(2, 3);
    println!("2 + 3 = {sum}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
