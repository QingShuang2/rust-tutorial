use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// Custom error type
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

fn read_and_parse_file(filename: &str) -> Result<i32, MyError> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    // Using Option
    let some_number = Some(42);
    let none_number: Option<i32> = None;

    println!("Some number: {:?}", some_number);
    println!("None number: {:?}", none_number);

    // Using Result
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Using ? operator
    match read_and_parse_file("number.txt") {
        Ok(num) => println!("Number from file: {}", num),
        Err(e) => println!("Error reading file: {:?}", e),
    }

    // Custom error
    let custom_result: Result<(), MyError> = Err(MyError::Custom("Something went wrong".to_string()));
    if let Err(e) = custom_result {
        println!("Custom error: {:?}", e);
    }
}