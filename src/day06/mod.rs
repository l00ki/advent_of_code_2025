use std::fs;
use std::str::FromStr;

enum Op {
    Add,
    Multiply,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Multiply),
            _ => Err(()),
        }
    }
}

struct Problem {
    operands: Vec<u64>,
    op: Op
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.op {
            Op::Add => self.operands.iter().sum(),
            Op::Multiply => self.operands.iter().product(),
        }
    }

    fn from_a(input: Vec<&str>) -> Self {
        let n = input.len();
        let mut operands = Vec::new();
        for i in 0..n-1 {
            operands.push(input[i].parse::<u64>().unwrap());
        }
        let op = input[n-1].parse::<Op>().unwrap();

        Self { operands, op }
    }

    fn from_b(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();
        let n_lines = lines.len() - 1;
        let n_ops = lines.iter().map(|l| l.len()).max().unwrap();
        let mut operands = vec![0; n_ops];

        for i in 0..n_ops {
            for j in 0..n_lines {
                if let Some(c) =  lines[j].chars().nth(i) {
                    if c != ' ' {
                        operands[i] *= 10;
                        operands[i] += c.to_string().parse::<u64>().unwrap();
                    }
                }
            }
        }

        let op_char = input.chars().filter(|c| !c.is_whitespace()).last();
        let op = match op_char {
            Some('+') => Op::Add,
            Some('*') => Op::Multiply,
            _ => panic!()
        };

        Self { operands, op }
    }
}

pub fn run() {
    let input_string = fs::read_to_string("inputs/06.txt").unwrap();
    let mut tokens = input_string.lines().map(|l| l.split_whitespace().collect()).collect::<Vec<Vec<&str>>>();
    let n_problems = tokens[0].len();

    let mut problems_a = Vec::new();
    for i in 0..n_problems {
        problems_a.push(Problem::from_a(tokens.iter().map(|t| t[i]).collect::<Vec<&str>>()));
    }

    let result_a: u64 = problems_a.iter().map(|p| p.solve()).sum();
    println!("06a: {result_a}");

    let mut problems_b = Vec::new();
    let lines = input_string.lines().collect::<Vec<&str>>();
    let n_lines = lines.len();
    let op_line = lines[n_lines-1];
    let mut start = 0;
    let mut end = 0;
    let mut op_chars = op_line.chars().skip(1);
    loop {
        match op_chars.next() {
            Some(' ') => end += 1,
            Some(c) => {
                let block = lines.iter()
                    .map(|l| l[start..end].to_owned())
                    .collect::<Vec<String>>()
                    .join("\n");
                problems_b.push(Problem::from_b(&block));
                if c == '\n' {
                    break;
                }
                start = end + 1;
                end = start;
            },
            None => {
                let block = lines.iter()
                    .map(|l| l[start..].to_owned())
                    .collect::<Vec<String>>()
                    .join("\n");
                problems_b.push(Problem::from_b(&block));
                break;
            }
        }
    }

    let result_b: u64 = problems_b.iter().map(|p| p.solve()).sum();
    println!("06b: {result_b}");

}
