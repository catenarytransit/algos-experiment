use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

const MAX: i32 = i32::MAX;

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<String, HashMap<String, i32>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: &str, connections: HashMap<String, i32>) {
        self.nodes.insert(node.to_string(), connections);
    }

    fn dijkstra(&self, start_node: &str, target_node: &str) -> Option<(i32, Vec<String>)> {
        let mut dists: HashMap<String, i32> = self.nodes.keys().map(|node| (node.clone(), MAX)).collect();
        let mut path: HashMap<String, String> = HashMap::new();
        let mut min_heap = BinaryHeap::new();

        dists.insert(start_node.to_string(), 0);
        min_heap.push(DijkstraNode { node: start_node.to_string(), dist: 0 });

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

#[derive(Eq, PartialEq)]
struct DijkstraNode {
    node: String,
    dist: i32,
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut graph = Graph::new();
    let mut node_a = HashMap::new();
    node_a.insert("b".to_string(), 9);
    node_a.insert("c".to_string(), 25);
    let mut node_b = HashMap::new();
    node_b.insert("a".to_string(), 10);
    node_b.insert("c".to_string(), 10);
    let mut node_c = HashMap::new();
    node_c.insert("a".to_string(), 9);
    node_c.insert("b".to_string(), 10);

    graph.add_node("a", node_a);
    graph.add_node("b", node_b);
    graph.add_node("c", node_c);

    if let Some((distance, path)) = graph.dijkstra("a", "c") {
        println!("Shortest Distance: {}", distance);
        println!("Shortest Path: {:?}", path);
    } else {
        println!("No path found");
    }
}
