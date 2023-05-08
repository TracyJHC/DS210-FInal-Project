extern crate plotters;
use plotters::prelude::*;

use crate::{read_data, clean_data, build_undirected_graph, connected_components};
use std::collections::HashMap;

pub fn visualize_results(file_path: &str) {
    // Read the ego-Facebook dataset
    let mut edges = read_data(file_path);
    clean_data(&mut edges);
    let graph = build_undirected_graph(&edges);

    let connected_components = connected_components(&graph);

    // Calculate the distribution of connected component sizes
    let mut size_distribution: HashMap<usize, usize> = HashMap::new();
    for component in &connected_components {
        let size = component.len();
        *size_distribution.entry(size).or_insert(0) += 1;
    }

    // Sort the distribution by component size
    let mut sorted_distribution: Vec<(usize, usize)> = size_distribution.into_iter().collect();
    sorted_distribution.sort_by_key(|entry| entry.0);

    let output_file = "output.svg";
    let width = 800;
    let height = 600;

    let root = SVGBackend::new(output_file, (width, height)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_size = sorted_distribution.iter().map(|entry| entry.0).max().unwrap();
    let max_count = sorted_distribution.iter().map(|entry| entry.1).max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Connected Component Size Distribution", ("Arial", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..max_size, 0..max_count)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            sorted_distribution
                .into_iter()
                .map(|(x, y)| Rectangle::new([(x, 0), (x + 1, y)], BLUE.mix(0.5).filled())),
        )
        .unwrap();
}
