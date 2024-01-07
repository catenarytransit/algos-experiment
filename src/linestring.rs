mod graph;
use graph::Node;
use std::time::Instant;
use graph::Graph;
use csv::Reader;
use core::fmt;

pub struct Mapped {
    node: Node, 
    edge_osm: u64, 
    edge_lon: f64, 
    edge_lat: f64,
    linestrings: Vec<String>,
}


impl fmt::Display for Mapped {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Point {:?} matches to {} at ({}, {}), which is part of {}", self.node, self.edge_osm, self.edge_lon, self.edge_lat, self.linestrings.join("")) 
    }
}


pub fn nearest_neighbor(node: Node, graph: &Graph) -> (u64, f64, f64) {
    let node_list: Vec<Node> = nodes_from_edges(graph);
    let tree = vpsearch::Tree::new(&node_list);
    let (index, _) = tree.find_nearest(&node);
    //let start_time = Instant::now();
    //println!("The nearest point, {}, is at ({}, {})\n Took {:?}ns", node_list[index].id, node_list[index].lat, node_list[index].lon, start_time.elapsed().as_nanos());
    (node_list[index].id, node_list[index].lat, node_list[index].lon)
}


pub fn nodes_from_edges(graph: &Graph) -> Vec<Node> {
    let edges = graph.edges.clone();
    let mut coords: Vec<Node> = Vec::new();
    for edge in edges {
        let edge_osm: u64 = edge.osm_id.parse::<u64>().unwrap();
        for point in edge.linestring {
            coords.push(Node{id: edge_osm, lon: point.0, lat: point.1});
        }
    }
    coords
}


pub fn get_linestrings(lat: f64, lon: f64) -> Vec<String> {
    let mut linestrings: Vec<String> = Vec::new();
    let coord = format!("{} {}", lon, lat);
    let mut rdr = Reader::from_path("testedges.csv").unwrap();
    for row in rdr.records() {
        let cell= row.unwrap();
        let linestring: &str = &cell[11];
        if linestring.contains(&coord) {
            linestrings.push(linestring.to_string().clone());
        }
    }
    linestrings
}


pub fn generate_match(graph: Graph) -> Vec<Mapped> {
    let mut map: Vec<Mapped> = Vec::new();
    let nodes = graph.nodes.clone();
    for node in nodes {
        let neighbor = nearest_neighbor(node, &graph);
        let linestrings = get_linestrings(neighbor.1, neighbor.2);
        map.push(Mapped{node, edge_osm: neighbor.0, edge_lon: neighbor.1, edge_lat: neighbor.2, linestrings});
    }
    map
}   


fn main() {
    println!("start");
    let start_time = Instant::now();

    let graph = Graph::from_csv("testedges.csv", "testnodes.csv");
    //let graph = Graph::from_csv_par3("edges.csv", "nodes.csv", 32);
    
    /* old x and y sort
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
    //nearest_neighbor(Node{id: 729462058, lon: -119.034311, lat: 33.4837658});
    println!("matched at t = {:?}", start_time.elapsed().as_nanos());
    
    for node_info in map {
        println!("{}", node_info);
    }
    

}
