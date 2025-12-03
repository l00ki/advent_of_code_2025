use std::fs;

pub fn run() {
    let input_string = fs::read_to_string("inputs/02.txt").unwrap();
    let range_strings = input_string.split(',');

    let mut result_a = 0;
    let mut result_b = 0;

    for range in range_strings {
        let bounds: Vec<&str> = range.split("-").map(|s| s.trim()).collect();
        let start = bounds[0].parse::<u64>().unwrap();
        let end = bounds[1].parse::<u64>().unwrap();
        for i in start..=end {
            if is_invalid_a(i) {
                result_a += i;
            }
            if is_invalid_b(i) {
                result_b += i;
            }
        }
    }

    println!("02a: {}", result_a);
    println!("02b: {}", result_b);
}

fn is_invalid_a(number: u64) -> bool {
    let s = format!("{}", number);
    let n = s.len();

    if n == 1 || n % 2 != 0 {
        return false;
    }
    let first_half = &s[..n/2];
    let second_half = &s[n/2..];

    first_half == second_half
}

fn is_invalid_b(number: u64) -> bool {
    let s = format!("{}", number);
    let n = s.len();

    for i in (1..n).filter(|i| n % i == 0) {
        let divisor = n / i;
        let pattern = &s[..i];

        if pattern.repeat(divisor) == s {
            return true;
        }
    }

    false
}

