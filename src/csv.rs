extern crate csv;
extern crate petgraph;
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};

use std::error::Error;
use std::fs::File;

#[derive(Debug, PartialEq, Clone)]
struct Edge {
    id: String,
    osm_id: u64,
    source: u64,
    target: u64,
    length: f64,
    foot: String,
    car_forward: String,
    car_backward: String,
    bike_forward: String,
    bike_backward: String,
    train: String,
    wkt: String,
}

#[derive(Debug, PartialEq, Clone)]
struct Node {
    id: String,
    lon: f64,
    lat: f64,
}

impl Eq for Node {}

impl Eq for Edge {}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open("edges.csv")?;
    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(file);

    // Create a directed graph with usize as the index type
    let mut graph = DiGraph::<Edge, f64, usize>::with_capacity(0, 0);

    // Iterate over CSV records and add edges to the graph
    for result in rdr.records() {
        let record = result?;
        let edge = Edge {
            id: record[0].to_string(),
            osm_id: record[1].parse()?,
            source: record[2].parse()?,
            target: record[3].parse()?,
            length: record[4].parse()?,
            foot: record[5].to_string(),
            car_forward: record[6].to_string(),
            car_backward: record[7].to_string(),
            bike_forward: record[8].to_string(),
            bike_backward: record[9].to_string(),
            train: record[10].to_string(),
            wkt: record[11].to_string(),
        };

        let source_node = NodeIndex::new((edge.source as u64).try_into().unwrap()       );
        let target_node = NodeIndex::new((edge.target as u64).try_into().unwrap());

        // Add nodes and edges to the graph
        graph.add_edge(source_node, target_node, edge.length);
    }

    // Find the shortest foot path using Dijkstra's algorithm
    let start_node = NodeIndex::new(1833121478); // Node 1
    let end_node = NodeIndex::new(1597291791); // Node 100
    let route = dijkstra(&graph, start_node, Some(end_node), |e| *e.weight());
    println!("{:#?}", route);

    Ok(())
}
