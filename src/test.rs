use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};
use std::fs::File;

use csv::ReaderBuilder;

const MAX: f64 = f64::MAX;

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<String, HashMap<String, f64>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: &str, connections: HashMap<String, f64>) {
        self.nodes.insert(node.to_string(), connections);
    }

    fn add_edge(&mut self, source: String, target: String, weight: f64) {
        self.nodes.get_mut(&source).unwrap().insert(target, weight);
    }

    fn dijkstra(&self, start_node: &str, target_node: &str) -> Option<(f64, Vec<String>)> {
        let mut dists: HashMap<String, f64> = self.nodes.keys().map(|node| (node.clone(), MAX)).collect();
        let mut path: HashMap<String, String> = HashMap::new();
        let mut min_heap = BinaryHeap::new();

        dists.insert(start_node.to_string(), 0.0);
        min_heap.push(DijkstraNode { node: start_node.to_string(), dist: 0.0 });

        while let Some(DijkstraNode { node, dist }) = min_heap.pop() {
            if &node == target_node {
                break;
            }

            if let Some(neighbors) = self.nodes.get(&node) {
                for (adj_node, weight) in neighbors.iter() {
                    let new_dist = dist + weight;
                    if new_dist < *dists.get(adj_node).unwrap_or(&MAX) {
                        dists.insert(adj_node.clone(), new_dist);
                        path.insert(adj_node.clone(), node.clone());
                        min_heap.push(DijkstraNode { node: adj_node.clone(), dist: new_dist });
                    }
                }
            }
        }

        if *dists.get(target_node).unwrap_or(&MAX) == MAX {
            return None;
        }

        let mut final_path = vec![target_node.to_string()];
        let mut current_node = target_node.to_string();
        while let Some(prev_node) = path.get(&current_node) {
            final_path.push(prev_node.clone());
            current_node = prev_node.clone();
        }

        final_path.reverse();

        Some((*dists.get(target_node).unwrap(), final_path))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct DijkstraNode {
    node: String,
    dist: f64,
}

impl Eq for DijkstraNode {}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.partial_cmp(&other.dist).unwrap_or(Ordering::Equal)
    }
}

fn main() {
    let mut graph = Graph::new();
    let file = File::open("edges.csv").unwrap();

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    // Iterate over the CSV records
    for result in rdr.records() {
        // Extract the record
        let record = result.unwrap();
        //let id: String = record[0].to_string();
        let source: String = record[2].to_string();
        let target: String = record[3].to_string();
        let length: f64 = record[4].parse().unwrap();
        if graph.nodes.contains_key(&source) {
            graph.add_edge(source, target, length);
            continue;
        }
        let mut node = HashMap::new();
        node.insert(target, length);
        graph.add_node(source.as_str(), node);
    }

    //println!("{:#?}", graph);
    if let Some((distance, path)) = graph.dijkstra("297523835", "364042999") {
        println!("Shortest Distance: {}", distance);
        println!("Shortest Path: {:?}", path);
    } else {
        println!("No path found");
    }
}
