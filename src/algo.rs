use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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
