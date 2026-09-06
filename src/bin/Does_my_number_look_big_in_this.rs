// Topic: Does_my_number_look_big_in_this
// Run with:  cargo run --bin Does_my_number_look_big_in_this

fn main() {
    println!("Hello World : cargo run --bin Does_my_number_look_big_in_this");
}

fn narcissistic(num: u64) -> bool {
    let x: Vec<u32> = num
        .to_string()
        .chars()
        .map(|a| a.to_digit(10).unwrap())
        .collect();

    let mut sum = 0;
    let p = x.len();

    for i in x {
        sum += (i as u64).pow(p as u32);
    }

    sum == num
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(input: u64, expected: bool) {
        let actual = narcissistic(input);
        assert_eq!(
            actual, expected,
            "\nIncorrect answer for n={}\nExpected: {expected}\nActual: {actual}",
            input
        )
    }

    #[test]
    fn basic_tests() {
        dotest(7, true);
        dotest(371, true);
        dotest(122, false);
        dotest(4887, false);
    }
}
