mod graph;
use graph::Node;
use graph::Edge;
use std::time::Instant;
use graph::Graph;
//use csv::Reader;


pub fn nearest_neighbor(node: Node, graph: &Graph) -> Node {
    let start_time = Instant::now();
    let node_list: Vec<Node> = graph.clone().get_nodes_in_edges();
    let tree = vpsearch::Tree::new(&node_list);
    let (index, _) = tree.find_nearest(&node);
    println!("Nearest point: {:?},  took {:?}ns", node_list[index], start_time.elapsed().as_nanos());
    node_list[index]
}

/*   realized this was really messy implimentation so now i repent
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
*/

pub fn generate_match(graph: Graph, node: Node) -> Vec<Edge> {
    let neighbor = nearest_neighbor(node, &graph);
    let edges: Vec<Edge> = graph.add_node_to_edges(&node, &neighbor);
    edges
}

fn main() {
    let mut start_time = Instant::now();
    let graph = Graph::from_csv("testedges.csv", "testnodes.csv");
    eprintln!("from_csv took {:?}", start_time.elapsed().as_secs_f64());
    
    let mynode = Node{id: 729462058, lon: -119.034311, lat: 33.4837658};
    let map = generate_match(graph, mynode);
    println!("matched at t = {:?}", start_time.elapsed().as_nanos());
    
    for node_info in map { println!("{:?}", node_info.linestring) };
    
}
