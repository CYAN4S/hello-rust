use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // 에러!

    println!("the first word is: {}", word);
}
