extern crate petgraph;
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::{Bfs, Visitable, VisitMap};
use std::collections::{HashSet, HashMap};

//Import three modules
mod data_processing;
mod path;
mod visualize;
use data_processing::{Edge, read_data, clean_data}; // Import the functions from the new module

fn build_undirected_graph(edges: &HashSet<Edge>) -> UnGraph<String, ()> {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut nodes = HashMap::new();

    for edge in edges {
        let source_node = *nodes.entry(edge.source.clone()).or_insert(graph.add_node(edge.source.clone()));
        let target_node = *nodes.entry(edge.target.clone()).or_insert(graph.add_node(edge.target.clone()));

        graph.add_edge(source_node, target_node, ());
    }

    graph
}
//BFS
fn bfs(graph: &UnGraph<String, ()>, start_node: NodeIndex) -> Vec<NodeIndex> {
    let mut visited = graph.visit_map();
    let mut bfs = Bfs::new(&graph, start_node);
    let mut result = Vec::new();

    while let Some(next_node) = bfs.next(&graph) {
        if !visited.is_visited(&next_node) {
            result.push(next_node);
            visited.insert(next_node.index()); // Use `index()` method to get the `usize` value
        }
    }

    result
}

fn connected_components(graph: &UnGraph<String, ()>) -> Vec<Vec<NodeIndex>> {
    let mut visited = graph.visit_map();
    let mut components: Vec<Vec<NodeIndex>> = Vec::new();

    for start_node in graph.node_indices() {
        if !visited.is_visited(&start_node) {
            let bfs_result = bfs(&graph, start_node);

            let mut component = Vec::new();
            for node in bfs_result {
                component.push(node);
                visited.visit(node);
            }

            components.push(component);
        }
    }

    components
}

fn main() {
    let file_path = "facebook_combined.txt"; 
    let mut edges = data_processing::read_data(file_path); 
    data_processing::clean_data(&mut edges); 

    let graph = build_undirected_graph(&edges);
    println!("Graph: {:?}", graph);

    // BFS
    let start_node = graph.node_indices().next().expect("Graph should have at least one node");
    let bfs_result = bfs(&graph, start_node);
    println!("BFS Result: {:?}", bfs_result);

    let components = connected_components(&graph);
    println!("Connected Components: {:?}", components);

    // Shortest Path
    let start_node = graph.node_indices().nth(0).expect("Graph should have at least one node");
    let end_node = graph.node_indices().nth(1).expect("Graph should have at least two nodes");

    let shortest_path_result = path::shortest_path(&graph, start_node, end_node);

    println!("Shortest Path Result: {:?}", shortest_path_result);
    
    let avg_path_length = path::avg_path_len(&graph);
    println!("Average Path Length: {:?}", avg_path_length);

    let file_path = "facebook_combined.txt";
    visualize::visualize_results(file_path);
}
