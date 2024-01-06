use std::time::Instant;
mod graph;
use graph::GTFSGraph;
fn main() {
    let start_time = Instant::now();
    let graph = GTFSGraph::from_file("gtfs_rail.zip", "f-9q5-metro~losangeles~rail");
    eprintln!("from_file took {:?}", start_time.elapsed().as_secs_f64());
    println!("{:#?}", graph);
}