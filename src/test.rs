use std::time::Instant;

mod graph;
use graph::{Graph, GTFSGraph};

fn main() {
    /*let threads = arguments::parse(std::env::args())
        .expect("Add --feeds <string>")
        .get::<u32>("threads").unwrap_or_else(|| 16.to_owned());
    let start_time = Instant::now();
    let graph = Graph::from_csv("testedges.csv", "testnodes.csv");
    eprintln!("from_csv took {:?}", start_time.elapsed().as_secs_f64());
    let start_time = Instant::now();
    let graph = Graph::from_csv_par3("testedges.csv", "testnodes.csv", threads);
    eprintln!("from_csv_par3 took {:?}", start_time.elapsed().as_secs_f64());
    let graph = Graph::from_csv_par4("testedges.csv", "testnodes.csv", threads);
    eprintln!("from_csv_par4 took {:?}", start_time.elapsed().as_secs_f64());
    
    let graph = Graph::from_csv_par4("edges.csv", "nodes.csv", threads);
    eprintln!("from_csv_par4 took {:?}", start_time.elapsed().as_secs_f64());*/
    
    let start_time = Instant::now();
    let gtfs_graph = GTFSGraph::from_file("gtfs_rail.zip", "f-9q5-metro~losangeles~rail");
    println!("GTFSGraph took {:?}", start_time.elapsed().as_secs_f64());
    println!("{:#?}", gtfs_graph);
}
