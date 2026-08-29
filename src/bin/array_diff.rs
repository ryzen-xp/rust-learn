// Topic: array_diff
// Run with:  cargo test --bin array_diff

use std::collections::HashSet;

fn main() {
    println!("Hello from : array_diff");

}

// Why `T: PartialEq + Eq + Hash`?
// `HashSet` stores values by hashing them and then checking equality for duplicates.
// So the element type must be able to:
// - compare with `==` (`PartialEq`)
// - have a proper equality rule (`Eq`)
// - produce a hash (`Hash`)
//
// In other words, Rust is saying: "This generic type can be used as a key in a HashSet."
// If `T` were a custom type, it would need to implement those traits too.
// For primitive types like `i32` and `String`, Rust already provides them.

pub fn array_diff<T: PartialEq + Eq + std::hash::Hash>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut hs = HashSet::new();

    for i in b {
        hs.insert(i);
    }

    let mut v = vec![];

    for x in a {
        if !hs.contains(&x) {
            v.push(x);
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
        assert_eq!(array_diff(vec![1, 2, 3], vec![1, 2]), vec![3]);
    }
}
