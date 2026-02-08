// For this lesson, we need to use Cargo for dependencies
// Create a Cargo.toml file with:
// [package]
// name = "lesson12"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// tokio = { version = "1", features = ["full"] }

use tokio;

#[tokio::main]
async fn main() {
    println!("Starting async lesson");

    // Basic async function
    async fn say_hello() {
        println!("Hello from async function!");
    }

    say_hello().await;

    // Async with return value
    async fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let result = add(5, 3).await;
    println!("5 + 3 = {}", result);

    // Concurrent tasks
    let task1 = tokio::spawn(async {
        println!("Task 1 starting");
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        println!("Task 1 finished");
        42
    });

    let task2 = tokio::spawn(async {
        println!("Task 2 starting");
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        println!("Task 2 finished");
        24
    });

    let (result1, result2) = tokio::try_join!(task1, task2).unwrap();
    println!("Results: {} and {}", result1, result2);

    // Futures
    use std::future::Future;
    use std::pin::Pin;

    fn create_future() -> Pin<Box<dyn Future<Output = i32>>> {
        Box::pin(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            100
        })
    }

    let future = create_future();
    let value = future.await;
    println!("Future result: {}", value);
}