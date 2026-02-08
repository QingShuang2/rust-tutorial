fn main() {
    // Lifetimes prevent dangling references
    let r;

    {
        let x = 5;
        r = &x; // Error: `x` does not live long enough
    }

    // println!("r: {}", r); // This would cause a compile error

    // Lifetime annotation syntax
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Struct with lifetimes
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("Important excerpt: {:?}", i);

    // Lifetime elision rules
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("First word: {}", word);

    // Static lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("Static string: {}", s);
}