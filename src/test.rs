use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;

use csv::ReaderBuilder;
use osmgraphing::configs::writing::network::graph;

const MAX: f64 = f64::MAX;

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Node {
    id: u64,
    lon: f64,
    lat: f64,
}

impl Node {
    fn new(id: u64, lon: f64, lat: f64) -> Self {
        Self {
            id: id,
            lon: lon,
            lat: lat,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Edge {
    id: String,
    osm_id: String,
    source: String,
    target: String,
    length: f64,
    foot: bool,
    car_forward: String,
    car_backward: String,
    bike_forward: bool,
    bike_backward: bool,
    train: String,
    linestring: Vec<(f64,f64)>,
}

impl Edge {
    fn new(id: String, osm_id: String, source: String, target: String, length: f64, foot: bool, car_forward: String, car_backward: String, bike_forward: bool, bike_backward: bool, train: String, linestring: Vec<(f64, f64)>) -> Self {
        Self {
            id: id,
            osm_id: osm_id,
            source: source,
            target: target,
            length: length,
            foot: foot,
            car_forward: car_forward,
            car_backward: car_backward,
            bike_forward: bike_forward,
            bike_backward: bike_backward,
            train: train,
            linestring: linestring,
        }
    }
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn from_csv_par(edge_file_path: &str, node_file_path: &str) -> Self {
        let mut graph = Arc::new(Mutex::new(Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }));
        /*let file = File::open(edge_file_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);
        for result in rdr.deserialize::<Edge>() {
            let edge: Edge = result.unwrap();
            graph.add_edge_obj(edge);
        }
        let file = File::open(node_file_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);
        for result in rdr.deserialize::<Node>() {
            let node: Node = result.unwrap();
            graph.add_node_obj(node);
        }*/
        let edges = File::open(edge_file_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(edges);
        let handles: Vec<_> = rdr.records().filter_map(|record| {
            match record {
                Ok(record) => Some(thread::spawn({
                    let shared_graph_clone = Arc::clone(&graph);
                    move || {
                        let edge = Edge {
                            id: record[0].to_string(),
                            osm_id: record[1].parse().unwrap(),
                            source: record[2].parse().unwrap(),
                            target: record[3].parse().unwrap(),
                            length: record[4].parse().unwrap(),
                            foot: if record[5].parse::<String>().unwrap() == "Allowed" {
                                true
                            } else {
                                false
                            },            
                            car_forward: record[6].to_string(),
                            car_backward: record[7].to_string(),
                            bike_forward: if record[8].parse::<String>().unwrap() == "Allowed" {
                                true
                            } else {
                                false
                            },  
                            bike_backward: if record[9].parse::<String>().unwrap() == "Allowed" {
                                true
                            } else {
                                false
                            },
                            train: record[10].to_string(),
                            linestring: record[11].to_string().trim_start_matches("LINESTRING(").trim_end_matches(')').split(", ")
                            .filter_map(|coord| {
                                let mut parts = coord.split_whitespace();
                                let lon_str = parts.next().unwrap();
                                let lat_str = parts.next().unwrap();
                                let lon: f64 = lon_str.parse().ok().unwrap();
                                let lat: f64 = lat_str.parse().ok().unwrap();
                                Some((lon, lat))
                            })
                            .collect()
                        };
                        let mut graph = shared_graph_clone.lock().unwrap();
                        graph.add_edge_obj(edge);
                    }
                })),
                Err(err) => {
                    eprintln!("Error reading record: {}", err);
                    None
                }
            }
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let nodes = File::open(node_file_path).expect("Failed to open file");
        let mut rdr = ReaderBuilder::new().from_reader(nodes);

        let handles: Vec<_> = rdr.records().filter_map(|record| {
            match record {
                Ok(record) => Some(thread::spawn({
                    let shared_graph_clone = Arc::clone(&graph);
                    move || {
                        let node = Node {
                            id: record[0].parse().unwrap(),
                            lon: record[1].parse().unwrap(),
                            lat: record[2].parse().unwrap()
                        };
                        let mut graph = shared_graph_clone.lock().unwrap();
                        graph.add_node_obj(node);
                    }
                })),
                Err(err) => {
                    eprintln!("Error reading record: {}", err);
                    None
                }
            }
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }
        println!("{:?}", graph);
        return graph.lock().unwrap().to_owned();
    }

    fn from_csv_par2(edge_file_path: &str, node_file_path: &str) -> Self {
        let mut graph = Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        };
        /*let file = File::open(edge_file_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);
        for result in rdr.deserialize::<Edge>() {
            let edge: Edge = result.unwrap();
            graph.add_edge_obj(edge);
        }
        let file = File::open(node_file_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);
        for result in rdr.deserialize::<Node>() {
            let node: Node = result.unwrap();
            graph.add_node_obj(node);
        }*/
        let edges = File::open(edge_file_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(edges);
        let handles: Vec<_> = rdr.records().filter_map(|record| {
            match record {
                Ok(record) => Some(thread::spawn({
                    move || {
                        Edge {
                            id: record[0].to_string(),
                            osm_id: record[1].parse().unwrap(),
                            source: record[2].parse().unwrap(),
                            target: record[3].parse().unwrap(),
                            length: record[4].parse().unwrap(),
                            foot: if record[5].parse::<String>().unwrap() == "Allowed" {
                                true
                            } else {
                                false
                            },            
                            car_forward: record[6].to_string(),
                            car_backward: record[7].to_string(),
                            bike_forward: if record[8].parse::<String>().unwrap() == "Allowed" {
                                true
                            } else {
                                false
                            },  
                            bike_backward: if record[9].parse::<String>().unwrap() == "Allowed" {
                                true
                            } else {
                                false
                            },
                            train: record[10].to_string(),
                            linestring: record[11].to_string().trim_start_matches("LINESTRING(").trim_end_matches(')').split(", ")
                            .filter_map(|coord| {
                                let mut parts = coord.split_whitespace();
                                let lon_str = parts.next().unwrap();
                                let lat_str = parts.next().unwrap();
                                let lon: f64 = lon_str.parse().ok().unwrap();
                                let lat: f64 = lat_str.parse().ok().unwrap();
                                Some((lon, lat))
                            })
                            .collect()
                        }
                    }
                })),
                Err(err) => {
                    eprintln!("Error reading record: {}", err);
                    None
                }
            }
        }).collect();

        for handle in handles {
            graph.add_edge_obj(handle.join().unwrap());
        }

        let nodes = File::open(node_file_path).expect("Failed to open file");
        let mut rdr = ReaderBuilder::new().from_reader(nodes);

        let handles: Vec<_> = rdr.records().filter_map(|record| {
            match record {
                Ok(record) => Some(thread::spawn({
                    move || {
                        Node {
                            id: record[0].parse().unwrap(),
                            lon: record[1].parse().unwrap(),
                            lat: record[2].parse().unwrap()
                        }
                    }
                })),
                Err(err) => {
                    eprintln!("Error reading record: {}", err);
                    None
                }
            }
        }).collect();

        for handle in handles {
            graph.add_node_obj(handle.join().unwrap());
        }
        graph
    }

    fn from_csv(edge_file_path: &str, node_file_path: &str) -> Self {
        let mut graph = Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        };
        /*let file = File::open(edge_file_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);
        for result in rdr.deserialize::<Edge>() {
            let edge: Edge = result.unwrap();
            graph.add_edge_obj(edge);
        }
        let file = File::open(node_file_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);
        for result in rdr.deserialize::<Node>() {
            let node: Node = result.unwrap();
            graph.add_node_obj(node);
        }*/
        let edges = File::open(edge_file_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(edges);

        for record in rdr.records() {
            let record = record.unwrap();
            let edge = Edge {
                id: record[0].to_string(),
                osm_id: record[1].parse().unwrap(),
                source: record[2].parse().unwrap(),
                target: record[3].parse().unwrap(),
                length: record[4].parse().unwrap(),
                foot: if record[5].parse::<String>().unwrap() == "Allowed" {
                    true
                } else {
                    false
                },            
                car_forward: record[6].to_string(),
                car_backward: record[7].to_string(),
                bike_forward: if record[8].parse::<String>().unwrap() == "Allowed" {
                    true
                } else {
                    false
                },  
                bike_backward: if record[9].parse::<String>().unwrap() == "Allowed" {
                    true
                } else {
                    false
                },
                train: record[10].to_string(),
                linestring: record[11].to_string().trim_start_matches("LINESTRING(").trim_end_matches(')').split(", ")
                .filter_map(|coord| {
                    let mut parts = coord.split_whitespace();
                    let lon_str = parts.next().unwrap();
                    let lat_str = parts.next().unwrap();
                    let lon: f64 = lon_str.parse().ok().unwrap();
                    let lat: f64 = lat_str.parse().ok().unwrap();
                    Some((lon, lat))
                })
                .collect()
            };
            graph.add_edge_obj(edge);
        }
        let nodes = File::open(node_file_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(nodes);
        for record in rdr.records() {
            let record = record.unwrap();
            let node = Node {
                id: record[0].parse().unwrap(),
                lon: record[1].parse().unwrap(),
                lat: record[2].parse().unwrap()
            };
            graph.add_node_obj(node);
        }
        graph
    }

    fn add_node(&mut self, id: u64, lon: f64, lat: f64) {
        self.nodes.push(Node::new(id, lon, lat));
    }

    fn add_node_obj(&mut self, node: Node) {
        self.nodes.push(node);
    }
    fn add_edge(&mut self, id: String, osm_id: String, source: String, target: String, length: f64, foot: bool, car_forward: String, car_backward: String, bike_forward: bool, bike_backward: bool, train: String, linestring: Vec<(f64, f64)>) {
        self.edges.push(Edge::new(id, osm_id, source, target, length, foot, car_forward, car_backward, bike_forward, bike_backward, train, linestring))
    }

    fn add_edge_obj(&mut self, edge: Edge) {
        self.edges.push(edge);
    }


}

fn main() {
    let mut graph = Graph::from_csv_par2("edges.csv", "nodes.csv");
}
