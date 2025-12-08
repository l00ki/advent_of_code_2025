use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input_string = fs::read_to_string("inputs/07.txt").unwrap();
    let lines: Vec<&str> = input_string.lines().collect();
    let n_cols = lines[0].len();

    let mut result_a = 0;
    let start_tachyon = lines[0].chars().position(|c| c == 'S').unwrap();
    let mut tachyons: HashMap<usize, u64> = HashMap::new();
    tachyons.insert(start_tachyon, 1);
    for line in lines {
        let mut new_tachyons: HashMap<usize, u64> = HashMap::new();
        for tachyon in tachyons.iter() {
            match line.chars().nth(*tachyon.0) {
                Some(c) => {
                    match c {
                        '.' | 'S' => {
                            new_tachyons.entry(*tachyon.0)
                                .and_modify(|t| *t += tachyon.1)
                                .or_insert(*tachyon.1);
                        },
                        '^' => {
                            result_a += 1;
                            if *tachyon.0 > 0 {
                                new_tachyons.entry(tachyon.0 - 1)
                                    .and_modify(|t| *t += tachyon.1)
                                    .or_insert(*tachyon.1);
                            }
                            if *tachyon.0 < n_cols - 1 {
                                new_tachyons.entry(tachyon.0 + 1)
                                    .and_modify(|t| *t += tachyon.1)
                                    .or_insert(*tachyon.1);
                            }
                        },
                        _ => panic!()
                    }
                },
                None => panic!()
            }
        }

        tachyons = new_tachyons;
    }

    let result_b: u64 = tachyons.iter().map(|t| t.1).sum();

    println!("07a: {result_a}");
    println!("07b: {result_b}");
}