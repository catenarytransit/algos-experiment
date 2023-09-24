use std::collections::HashMap;

fn main() {
    let mut graph = HashMap::new();

    // Insert a nested HashMap
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

    for (key, value) in &graph {
        println!("{}: {:?}", key, value);
    }
}
