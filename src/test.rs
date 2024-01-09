use std::time::Instant;

mod graph;
use graph::Graph;

fn main() {
    let threads = arguments::parse(std::env::args())
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
}
