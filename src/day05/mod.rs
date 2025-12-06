use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

pub fn run() {
    let input_string = fs::read_to_string("inputs/05.txt").unwrap();
    let lines = input_string.lines().collect::<Vec<&str>>();

    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut line_iter = lines.iter();
    while let Some(line) = line_iter.next() && !line.is_empty() {
        let words: Vec<&str> = line.split('-').collect();
        let lb: u64 = words[0].parse().unwrap();
        let ub: u64 = words[1].parse().unwrap();
        ranges.push(lb..=ub);
    }

    let mut ids: HashSet<u64> = HashSet::new();
    while let Some(line) = line_iter.next() {
        let id: u64 = line.parse().unwrap();
        ids.insert(id);
    }

    let result_a = ids.iter().filter(|i| ranges.iter().any(|range| range.contains(i))).count();
    println!("05a: {result_a}");

    let mut modified = true;
    let mut processed_ranges: Vec<RangeInclusive<u64>> = ranges.iter().map(|r| r.clone()).collect();
    while modified {
        modified = false;
        let n = processed_ranges.len();
        for i in 0..n {
            for j in i+1..n {
                let range_i = processed_ranges[i].clone();
                let range_j = processed_ranges[j].clone();

                if range_i.start() <= range_j.end() && range_i.end() >= range_j.start() {
                    let lb = range_i.start().min(range_j.start()).clone();
                    let ub = range_i.end().max(range_j.end()).clone();

                    processed_ranges[i] = lb..=ub;
                    processed_ranges.remove(j);

                    modified = true;
                    break;
                }
            }
            if modified {
                break;
            }
        }
    }

    let result_b: u64 = processed_ranges.iter().map(|r| count_range(r)).sum();
    println!("05b: {result_b}");
}

fn count_range(range: &RangeInclusive<u64>) -> u64 {
    range.end() - range.start() + 1
}