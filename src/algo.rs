use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};

fn dijsktra(&graph: HashMap<>, start_node: usize, target_node: usize) -> Option<(i32, Vec<usize>)> {
    let mut dists: Vec<i32> = vec![MAX; graph.n];
    let mut done: Vec<bool> = vec![false; graph.n];
    dists[start_node] = 0;
    let mut path: Vec<usize> = vec![0; graph.n];
    for _ in 0..(graph.n - 1) {
        let min_node = graph.min_distance(&mut dists, &mut done);
        done[min_node] = true;
        for adj_node in 0..graph.n {
            let not_zero = graph[min_node][adj_node] != 0;
            let new_dist = dists[min_node] + graph[min_node][adj_node];
            if !done[adj_node]
                && not_zero
                && dists[min_node] != MAX
                && new_dist < dists[adj_node]
            {
                path[adj_node] = min_node;
                dists[adj_node] = new_dist;
            }
        }
    }
    if dists[target_node] == MAX {
        return None;
    }
    let mut target_path = path[target_node];
    let mut final_path = vec![target_node];
    while !final_path.contains(&0) {
        final_path.push(target_path);
        target_path = path[target_path];
    }
    final_path.reverse();
    return Some((dists[target_node], final_path));
}



fn main() {
    let mut graph = HashMap::new();

    let mut links = HashMap::new();
    links.insert("b", 10);
    links.insert("c", 10);
    
    graph.insert("a", links);

    let mut links = HashMap::new();
    links.insert("a", 10);
    links.insert("c", 10);
    
    graph.insert("b", links);

    let mut links = HashMap::new();
    links.insert("b", 10);
    links.insert("a", 10);
    
    graph.insert("c", links);

    let serialized = serde_json::to_string(&graph).expect("Serialization failed");
    
    let mut file = File::create("graph.json").expect("File creation failed");
    file.write_all(serialized.as_bytes()).expect("Write failed");

    for (key, value) in &graph {
        println!("{}: {:?}", key, value);
    }
}
