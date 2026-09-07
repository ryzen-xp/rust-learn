// Topic: multioles_3_and_5
// Run with:  cargo run --bin multioles_3_and_5

fn main() {
    println!("Hello World : cargo run --bin multioles_3_and_5");
}

fn solution(mut num: i32) -> i32 {
    num -= 1;

    if num < 0 {
        return 0;
    }

    let mut sum = 0;

    while num != 0 {
        if num % 3 == 0 || num % 5 == 0 {
            sum += num;
        }

        num -= 1;
    }

    sum
}

mod tests {
    use super::solution;

    #[test]
    fn sample_tests() {
        // assertion(expected, input);
        assertion(23, 10);
        assertion(33, 11);
        assertion(225, 33);
        assertion(8, 6);
        assertion(3420, 123);
        assertion(543, 50);
        assertion(0, 0);
        assertion(0, -203);
        assertion(25719750, 10500);
    }

    fn assertion(expected: i32, input: i32) {
        let actual = solution(input);

        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n",
            expected,
            actual,
            input
        );
    }
}
