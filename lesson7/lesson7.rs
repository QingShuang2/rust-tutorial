use std::collections::HashMap;

fn main() {
    // Vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("Vector: {:?}", v);

    let v2 = vec![1, 2, 3, 4, 5];
    println!("Vector 2: {:?}", v2);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Iterating over vectors
    for i in &v2 {
        println!("Element: {}", i);
    }

    // Mutable iteration
    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
    }
    println!("Modified vector: {:?}", v3);

    // Using enums in vectors
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Spreadsheet row: {:?}", row);

    // Strings
    let mut s = String::new();
    s.push_str("Hello");
    s.push(' ');
    s.push_str("world!");
    println!("String: {}", s);

    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");
    println!("String 2: {}", s2);
    println!("String 3: {}", s3);

    let s4 = s2 + &s3; // s2 is moved here
    println!("Combined string: {}", s4);

    let s5 = format!("{}-{}-{}", "tic", "tac", "toe");
    println!("Formatted string: {}", s5);

    // Iterating over strings
    for c in "नमस्ते".chars() {
        println!("Char: {}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("Byte: {}", b);
    }

    // HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue score: {:?}", score);

    // Iterating over HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating HashMap
    scores.insert(String::from("Blue"), 25);
    println!("Updated scores: {:?}", scores);

    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(30);
    println!("Scores with entry: {:?}", scores);

    // Counting words
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word counts: {:?}", map);
}