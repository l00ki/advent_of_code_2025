use std::cmp::{max, min};
use std::fs;
use std::ops::Range;

pub fn run() {
    let input = fs::read_to_string("inputs/09.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let n_v = lines.len();
    let mut vs: Vec<(u64, u64)> = Vec::with_capacity(n_v);

    for line in lines {
        let words: Vec<&str> = line.split(',').collect();
        vs.push((words[0].parse().unwrap(), words[1].parse().unwrap()));
    }

    let mut rects: Vec<Rect> = Vec::new();
    for i in 0..n_v {
        for j in i+1..n_v {
            let rect = Rect::from(vs[i], vs[j]);
            rects.push(rect);
        }
    }

    rects.sort_by(|a, b| a.area().cmp(&b.area()).reverse());
    let rect = rects.first().unwrap();
    let result_a = rect.area();
    println!("09a: {result_a}");

    let mut edges: Vec<Rect> = Vec::new();
    for i in 0..n_v {
        let idx = (i + 1) % n_v;
        edges.push(Rect::from(vs[i], vs[idx]));
    }

    let mut result_b = 0;
    for rect in rects {
        let inner = rect.inner();
        if !edges.iter().any(|e| e.overlaps(&inner)) {
            result_b = rect.area();
            break;
        }
    }

    println!("09b: {result_b}");
}

struct Line(Range<u64>);

impl Line {
    fn overlaps(&self, other: &Self) -> bool {
        max(self.0.start, other.0.start) < min(self.0.end, other.0.end)
    }

    fn length(&self) -> u64 {
        self.0.start.abs_diff(self.0.end)
    }
}

struct Rect {
    x: Line,
    y: Line,
}

impl Rect {
    fn from(a: (u64, u64), b: (u64, u64)) -> Self {
        Self {
            x: Line(min(a.0, b.0) .. max(a.0, b.0) + 1),
            y: Line(min(a.1, b.1) .. max(a.1, b.1) + 1)
        }
    }

    fn inner(&self) -> Self {
        Self {
            x: Line(self.x.0.start + 1 .. self.x.0.end - 1),
            y: Line(self.y.0.start + 1 .. self.y.0.end - 1)
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.x.overlaps(&other.x) && self.y.overlaps(&other.y)
    }

    fn area(&self) -> u64 {
        self.x.length() * self.y.length()
    }
}
