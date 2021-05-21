fn main() {
    let mut s = String::from("hello world");
    let first = first_word(&s);
    let second = second_word(&s);
    println!("first {}", first);
    println!("second {}", second);
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..s.len()];
        }
    }
    &s[..]
}