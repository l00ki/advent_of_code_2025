use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vertex(i32, i32, i32);

impl FromStr for Vertex {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(',').take(3);
        if let (Some(word_x), Some(word_y), Some(word_z)) = (words.next(), words.next(), words.next()) {
            if let (Ok(x), Ok(y), Ok(z)) = (word_x.parse(), word_y.parse(), word_z.parse()) {
                Ok(Self(x, y, z))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl Vertex {
    fn distance(&self, other: &Vertex) -> f32 {
        let dx = (self.0 - other.0) as f32;
        let dy = (self.1 - other.1) as f32;
        let dz = (self.2 - other.2) as f32;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Edge(Vertex, Vertex);

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length().partial_cmp(&other.length())
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Edge {
    fn length(&self) -> f32 {
        self.0.distance(&self.1)
    }

    fn connects_to(&self, other: &Vertex) -> bool {
        self.0 == *other || self.1 == *other
    }
}

#[derive(Clone, PartialEq)]
struct Graph {
    vertices: HashSet<Vertex>,
    edges: HashSet<Edge>
}

impl Graph {
    fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    fn insert_vertex(&mut self, v: Vertex) -> bool {
        self.vertices.insert(v)
    }

    fn insert_edge(&mut self, e: Edge) -> bool {
        self.insert_vertex(e.0);
        self.insert_vertex(e.1);
        self.edges.insert(e)
    }

    fn insert_graph(&mut self, g: &Graph) -> bool {
        g.vertices.iter().all(|v| self.insert_vertex(*v))
        || g.edges.iter().all(|e| self.insert_edge(*e))
    }

    fn contains(&self, e: &Edge) -> bool {
        self.edges.contains(e)
    }

    fn connects_to(&self, e: &Edge) -> bool {
        self.vertices.iter().any(|a| e.connects_to(a))
    }

    fn len(&self) -> usize {
        self.vertices.len()
    }
}

pub fn run() {
    let input = fs::read_to_string("inputs/08.txt").unwrap();
    let mut vertices: Vec<Vertex> = Vec::new();
    for line in input.lines() {
        vertices.push(line.parse().unwrap());
    }

    let mut edges: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    for (i, a) in vertices.iter().enumerate() {
        for b in vertices.iter().skip(i + 1) {
            edges.push(Reverse(Edge(*a, *b)));
        }
    }

    let mut circuits: Vec<Graph> = Vec::new();
    for v in vertices {
        let mut graph = Graph::new();
        graph.insert_vertex(v);
        circuits.push(graph);
    }

    let mut result_a = 0;
    let mut i = 0;
    while let Some(edge) = edges.pop() && circuits.len() > 1 {
        i += 1;
        let mut connecting_circuits = circuits.iter().filter(|c| c.connects_to(&edge.0));
        let circuit_a = connecting_circuits.next();
        let circuit_b = connecting_circuits.next();
        let mut idx_a = None;
        if let Some(circuit) = circuit_a {
            idx_a = Some(circuits.iter().position(|c| c == circuit).unwrap());
        }

        let mut idx_b = None;
        if let Some(circuit) = circuit_b {
            idx_b = Some(circuits.iter().position(|c| c == circuit).unwrap());
        }

        circuits[idx_a.unwrap()].insert_edge(edge.0);
        if let Some(idx) = idx_b {
            let circuit_b = circuits[idx].clone();
            circuits[idx_a.unwrap()].insert_graph(&circuit_b);
            circuits.remove(idx);
        }

        if i == 1000 {
            circuits.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
            result_a = circuits.iter().take(3).map(|c| c.len() as u32).product();
            println!("08a: {result_a}");
        }

        if circuits.len() == 1 {
            let result_b: i64 = edge.0.0.0 as i64 * edge.0.1.0 as i64;
            println!("08b: {result_b}");
        }
    }
}