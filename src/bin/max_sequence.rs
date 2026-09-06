// Topic: max_sequence
// Run with:  cargo run --bin max_sequence

fn main() {
    println!("Hello World : cargo run --bin max_sequence");

    assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(max_sequence(&[11]), 11);
    assert_eq!(max_sequence(&[-32]), 0);

    println!("PASS");
}

use std::cmp::max;

fn max_sequence(seq: &[i32]) -> i32 {
    if seq.is_empty() {
        return 0;
    }

    let mut current = 0;
    let mut best = 0;

    for &value in seq {
        current = max(0, current + value);
        best = max(best, current);
    }

    best
}

fn _max_sequence(seq: &[i32]) -> i32 {
    let mut m = 0;
    seq.iter().fold(0, |prev, &v| {
        let p = v.max(prev + v);
        m = m.max(p);
        p
    });
    m
}
