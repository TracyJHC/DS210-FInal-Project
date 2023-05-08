use petgraph::algo::dijkstra;
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::{EdgeRef};
use std::collections::HashMap;

pub fn shortest_path(graph: &UnGraph<String, ()>, start_node: NodeIndex, end_node: NodeIndex) -> Option<Vec<NodeIndex>> {
    let shortest_path_lengths: HashMap<NodeIndex, f64> = dijkstra(graph, start_node, Some(end_node), |_| 1.0);
    if let Some(path_length) = shortest_path_lengths.get(&end_node) {
        let mut path = Vec::new();
        let mut current_node = end_node;

        while current_node != start_node {
            path.push(current_node);
            for edge in graph.edges(current_node) {
                let source = edge.source();
                let target = edge.target();
                let neighbor = if source == current_node { target } else { source };
                if let Some(neighbor_path_length) = shortest_path_lengths.get(&neighbor) {
                    if *neighbor_path_length == *path_length - 1.0 {
                        current_node = neighbor;
                        break;
                    }
                }
            }
        }
        path.push(start_node);
        path.reverse();
        Some(path)
    } else {
        None
    }
}

pub fn avg_path_len(graph: &UnGraph<String, ()>) -> f32 {
    let mut sum = 0;
    let node_count = graph.node_count();
    
    for start_node in graph.node_indices() {
        let shortest_path_lengths: HashMap<NodeIndex, f64> = dijkstra(graph, start_node, None, |_| 1.0);

        for (end_node, path_length) in &shortest_path_lengths {
            if start_node != *end_node {
                sum += *path_length as usize;
            }
        }
    }

    sum as f32 / (node_count * (node_count - 1)) as f32
}