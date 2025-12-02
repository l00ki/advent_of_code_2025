mod rotation;
mod lock;

use crate::day01::lock::Lock;

use std::fs;

pub fn run() {
    let mut lock = Lock::default();

    let input_string = fs::read_to_string("inputs/01.txt").unwrap();
    let input_lines = input_string.lines();
    for input_line in input_lines {
        let rotation = input_line.parse().unwrap();
        lock.rotate(rotation);
    }

    println!("01a: {}", lock.num_final_clicks);
    println!("01b: {}", lock.num_any_clicks);
}
