fn main() {
    let s = String::from("Hello World");
    let t = first_word(&s);

    println!("First word: {}", t);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
