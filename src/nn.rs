mod graph;
use graph::Graph;
use graph::Node;
use std::time::Instant;

pub fn nearest_neighbor(node: Node) -> (u64, f64, f64) {
    let start_time = Instant::now();
    //let graph = Graph::from_csv_par3("edges.csv", "nodes.csv", 32);
    let graph = Graph::from_csv("testedges.csv", "testnodes.csv");
    eprintln!("from_csv took {:?}", start_time.elapsed().as_secs_f64());
    let node_list = nodes_from_edges(graph);
    let tree = vpsearch::Tree::new(&node_list);
    let (index, _) = tree.find_nearest(&node);
    let start_time = Instant::now();
    println!("The nearest point, {}, is at ({}, {})\n Took {:?}ns", node_list[index].id, node_list[index].lat, node_list[index].lon, start_time.elapsed().as_nanos());
    (node_list[index].id, node_list[index].lat, node_list[index].lon)
}

pub fn nodes_from_edges(graph: Graph) -> Vec<Node> {
    let edges = graph.edges;
    let mut coords: Vec<Node> = Vec::new();
    for edge in edges {
        let edge_id: u64 = edge.id.parse::<u64>().unwrap();
        for point in edge.linestring {
            coords.push(Node{id: edge_id, lon: point.0, lat: point.1});
        }
    }
    coords
}

fn main() {
    nearest_neighbor(Node{id: 729462058, lon: -119.034311, lat: 33.4837658});
}