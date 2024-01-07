mod graph;
use graph::Node;
use std::time::SystemTime;
use graph::Graph;
mod nn;

fn generate_match(graph: Graph) -> Vec<(Node, (u64, f64, f64))> {
    let mut map: Vec<(Node, (u64, f64, f64))> = Vec::new();
    for node in graph.nodes {
        map.push((node, nn::nearest_neighbor(node)));
    }
    map
}   

fn main() {
    println!("start");
    let start = SystemTime::now();

    let graph = Graph::from_csv("testedges.csv", "testnodes.csv");
    //let graph = Graph::from_csv_par3("edges.csv", "nodes.csv", 32);

    /* unused sorting by x and y  
    let mut sort_x = graph.nodes.clone();
    sort_x.sort_by(|a, b| a.lon.partial_cmp(&b.lon).unwrap());
    let mut sort_y = graph.nodes.clone();
    sort_y.sort_by(|a, b| a.lat.partial_cmp(&b.lat).unwrap());
    
    let parsed = SystemTime::now().duration_since(start).expect("Clock may have gone backwards");
    println!("parsed at t = {:?}", parsed);

    let iter_x = sort_x.iter();
    let iter_y = sort_y.iter();
    
    println!("aaa {}", 	iter_x.len());
    
    for node in iter_x {
    	println!("point {:?}", node);
    }
    
    for node in iter_y {
    	println!("point {:?}", node);
    }
    */
   
    let map = generate_match(graph);

    let matched = SystemTime::now().duration_since(start).expect("error");
    println!("matched at t = {:?}", matched);

    //for (point, linestring) in map {
    //    println!("point {:?} matches to linestring {:?} at geodesic {:?}", point, linestring, subsection);
    //}
}
