use std::fs;

pub fn run() {
    let mut result_a = 0;
    let mut result_b = 0;

    let input_string = fs::read_to_string("inputs/04.txt").unwrap();
    let lines = input_string.lines().collect::<Vec<&str>>();

    let mut roll_coords: Vec<(u32, u32)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '@' {
                roll_coords.push((i as u32, j as u32));
            }
        }
    }
    result_a = roll_coords.iter().map(|x| count_neighbors(x, &roll_coords)).filter(|n| *n < 4).count();

    println!("04a: {result_a}");

    loop {
        let taken: Vec<&(u32, u32)> = roll_coords.iter().filter(|x| count_neighbors(x, &roll_coords) < 4).collect();
        let n_taken = taken.len();
        if n_taken == 0 {
            break;
        }
        result_b += n_taken;

        let mut new_roll_coords = roll_coords.clone();
        new_roll_coords.retain(|x| !taken.iter().any(|y| *y == x));
        roll_coords = new_roll_coords;
    }

    println!("04b: {result_b}");
}

fn count_neighbors(x0: &(u32, u32), coords: &[(u32, u32)]) -> u32 {
    coords.iter().filter(|x| distance(x, x0) < 2.0).count() as u32 - 1
}

fn distance(x0: &(u32, u32), x1: &(u32, u32)) -> f32 {
    let result = ((x0.0 as f32 - x1.0 as f32).powf(2.0) + (x0.1 as f32 - x1.1 as f32).powf(2.0)).sqrt();
    result
}