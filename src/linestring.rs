mod graph;
use graph::Node;
use graph::Edge;
use std::collections::HashMap;
use std::time::SystemTime;
use graph::Graph;

//extracts coordinates as tuples from linestring
/*fn coords(linestring: Vec<f64, f64>) -> Vec<(f64, f64)> {
    let coords: Vec<(f64, f64)> = linestring.to_string().trim_start_matches("LINESTRING(").trim_end_matches(')').split(", ")
    .filter_map(|coord| {
        let mut parts = coord.split_whitespace();
        let lon_str = parts.next().unwrap();
        let lat_str = parts.next().unwrap();
        let lon: f64 = lon_str.parse().ok().unwrap();
        let lat: f64 = lat_str.parse().ok().unwrap();
        Some((lon, lat))
    })
    .collect();
    coords
}*/

//let mut subsection: (f64, f64) = (None, None);
//takes in a Node and returns the linestring closest to point
fn nearest_neighbor(node: Node) {//->  Vec<String> {
    //geographic-lib nearest neighbor written only in c++ (using vpsearch and geodesic distance)
    //however we have vincenty from geographiclib-rs and vpsearch in rust so we can try to recreate it
}

fn generate_match() -> HashMap<Node, String>{
    let mut map = HashMap::new();
    //for node in node_list {
    //    map.insert(node, get_linestring(node));
    //}
    //map.insert(node_list.get(0), nearest_neighbor(node_list.get(0)));
    map
}   

fn main() {
    println!("start");
    let start = SystemTime::now();

    let graph = Graph::from_csv("testedges.csv", "testnodes.csv");

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

    let edges = graph.edges;
    println!("\t\t\t{:?}", &edges[0].linestring);
    
    for node in iter_y {
    	println!("point {:?}", node);
    }
    //let map = generate_match(); //main line of code that does things

    //let matched = SystemTime::now().duration_since(start).expect("Clock may have gone backwards");
    //println!("matched at t = {:?}", matched);

    //for (point, linestring) in map {
        //println!("point {:?} matches to linestring {:?} at geodesic {:?}", point, linestring, subsection);
    //}
}
