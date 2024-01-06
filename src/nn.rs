mod graph;
use std::time::Instant;
use graph::Graph;

use graph::Node;

fn main() {
    let start_time = Instant::now();
    //let graph = Graph::from_csv_par3("edges.csv", "nodes.csv", 32);
    let graph = Graph::from_csv("testedges.csv", "testnodes.csv");
    eprintln!("from_csv_par3 took {:?}", start_time.elapsed().as_secs_f64());
    println!("Graph: {:#?}", graph);
    let tree = vpsearch::Tree::new(&graph.nodes);
    let (index, _) = tree.find_nearest(&Node {
        id: 2729462058,
        lon: -119.0343115,
        lat: 33.4837658,
    });
    let start_time = Instant::now();
    println!("The nearest point, {}, is at ({}, {})\n Took {:?}", graph.nodes[index].id, graph.nodes[index].lat, graph.nodes[index].lon, start_time.elapsed().as_millis());
}