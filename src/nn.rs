mod graph;
use std::time::Instant;
use graph::Graph;

use graph::Node;

fn main() {
    let start_time = Instant::now();
    let graph = Graph::from_csv_par3("edges.csv", "nodes.csv", 32);
    eprintln!("from_csv_par3 took {:?}", start_time.elapsed().as_secs_f64());
    let tree = vpsearch::Tree::new(&graph.nodes);
    let (index, _) = tree.find_nearest(&Node {
        id: 8323837736,
        lon: -117.2544123,
        lat: 34.1165449,
    });
    println!("The nearest point is at ({}, {})", graph.nodes[index].lat, graph.nodes[index].lon);
}