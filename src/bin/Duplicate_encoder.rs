// Topic: Duplicate_encoder
// Run with:  cargo run --bin Duplicate_encoder

use std::collections::HashMap;

fn main() {
    let word = "Success";
    assert_eq!(duplicate_encode(word), ")())())");
    assert_eq!(duplicate_encode("TUna"), "((((");

    println!("PASS");
}

fn duplicate_encode(word: &str) -> String {
    let mut counter = HashMap::new();

    for c in word.chars() {
        let normalized = c.to_ascii_lowercase();

        *counter.entry(normalized).or_insert(0) += 1;
    }

    let y = word
        .chars()
        .map(|x| {
            if *counter.entry(x.to_ascii_lowercase()).or_default() >= 2 {
                ')'
            } else {
                '('
            }
        })
        .collect();

    y
}
