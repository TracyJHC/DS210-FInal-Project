use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Edge {
    pub source: String,
    pub target: String,
}

pub fn read_data(file_path: &str) -> HashSet<Edge> {
    let file = File::open(file_path).expect("Unable to open the file");
    let reader = BufReader::new(file);
    let mut edges = HashSet::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let nodes: Vec<&str> = line.split(' ').collect(); // Change this line to split by space

        if nodes.len() == 2 {
            let edge = Edge {
                source: nodes[0].to_string(),
                target: nodes[1].to_string(),
            };
            edges.insert(edge);
        }
    }

    edges
}

pub fn is_valid_edge(edge: &Edge) -> bool {
    // You can add any additional validation checks for the edges here
    !edge.source.is_empty() && !edge.target.is_empty()
}

pub fn clean_data(edges: &mut HashSet<Edge>) {
    // Clean the data (remove duplicates, invalid edges, and errors)
    edges.retain(|edge| is_valid_edge(edge));
}