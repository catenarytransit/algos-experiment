use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::fs::File;

use csv::{ReaderBuilder};

const MAX: f64 = f64::MAX;

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    train: String
    //linestring: String,
}

impl Edge {
    fn new(id: String, osm_id: String, source: String, target: String, length: f64, foot: bool, car_forward: String, car_backward: String, bike_forward: bool, bike_backward: bool, train: String) -> Self {
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

    fn add_node(&mut self, id: u64, lon: f64, lat: f64) {
        self.nodes.push(Node::new(id, lon, lat));
    }

    fn add_node_obj(&mut self, node: Node) {
        self.nodes.push(node);
    }
    fn add_edge(&mut self, id: String, osm_id: String, source: String, target: String, length: f64, foot: bool, car_forward: String, car_backward: String, bike_forward: bool, bike_backward: bool, train: String) {
        self.edges.push(Edge::new(id, osm_id, source, target, length, foot, car_forward, car_backward, bike_forward, bike_backward, train))
    }

    fn add_edge_obj(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
    

}

fn main() {
    let mut graph = Graph::new();
    let thread_pool = ThreadPoolBuilder::new().num_threads(16).build().unwrap();
    let edges = File::open("edges.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(edges);
    let records_iterator = rdr.records();
    let edges: Vec<_> = thread_pool.install(|| {
        records_iterator
        .par_bridge()
        .map(|record| {
            let record = record.unwrap();
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
            }
        }).collect()
    });
    for edge in edges {
        graph.add_edge_obj(edge);
    }

    let nodes = File::open("nodes.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(nodes);
    let records_iterator = rdr.records();
    let nodes: Vec<_> = thread_pool.install(|| {
        records_iterator
            .par_bridge()
            .map(|record| {
                let record = record.unwrap();
                Node {
                    id: record[0].parse().unwrap(),
                    lon: record[1].parse().unwrap(),
                    lat: record[2].parse().unwrap(),
                }
            })
            .collect()
    });
    for node in nodes {
        graph.add_node_obj(node);
    }

}
