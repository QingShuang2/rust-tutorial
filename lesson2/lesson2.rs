fn main() {
    println!("Lesson 2: Ownership and Borrowing");

    let name = String::from("Learner");
    greet(&name);

    let len = count_chars(&name);
    println!("Name length: {len}");

    let tagged = add_tag(name);
    println!("Tagged: {tagged}");

    let first = first_word(&tagged);
    println!("First word: {first}");

    let original = String::from("score");
    let copy = original.clone();
    println!("Original: {original}, Copy: {copy}");
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn count_chars(s: &str) -> usize {
    s.chars().count()
}

fn add_tag(mut s: String) -> String {
    s.push_str(" [v1]");
    s
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
