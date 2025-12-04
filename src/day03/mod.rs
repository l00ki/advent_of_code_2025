use std::fs;

pub fn run() {
    let mut result_a = 0;
    let mut result_b = 0;

    let input_string = fs::read_to_string("inputs/03.txt").unwrap();
    let lines = input_string.lines();

    for line in lines {
        let batteries: Vec<u32> =
            line.chars().map(|a| a.to_string().parse::<u32>().unwrap()).collect();

        result_a += find_max_joltage(&batteries, 2);
        result_b += find_max_joltage(&batteries, 12);
    }

    println!("03a: {result_a}");
    println!("03b: {result_b}");
}

fn find_max_joltage(batteries: &Vec<u32>, n_choose: usize) -> u64 {
    let n = batteries.len();
    let mut chosen: Vec<u32> = vec!();
    let mut lb = 0;

    while chosen.len() < n_choose {
        let ub = n - n_choose + chosen.len() + 1;
        let next = batteries[lb..ub].iter().max().unwrap();
        lb += batteries[lb..ub].iter().position(|i| i == next).unwrap() + 1;

        chosen.push(*next);
    }

    let mut result: u64 = 0;
    let mut multiplicator: u64 = 1;
    while let Some(x) =  chosen.pop() {
        result += multiplicator * x as u64;
        multiplicator *= 10;
    }

    result
}