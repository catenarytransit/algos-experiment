use std::error::Error;
use csv::Reader;
use std::collections::HashMap;
use std::time::SystemTime;
use regex::Regex

#[derive(Debug, serde::Deserialize)]
struct Node {
    id: i64,
    lon: f64,
    lat: f64,
}

#[derive(Debug, serde::Deserialize)]
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
    linestring: str,
}

let mut node_list: Vec<Node> = Vec::new();
let mut edge_list: Vec<Edge> = Vec::new();

fn parse_nodes() {
    let mut read = Reader::from_path("../testnodes.csv")?;
    let mut list = read.deserialize();
    for item in list.records() {
        if let Some(result) = list.next() {
            node_list.insert.push(Node = result?);
        }
    }
}

fn parse_edges() {
    let mut read = Reader::from_path("../testedges.csv")?;
    let mut list = read.deserialize();
    for item in list.records() {
        if let Some(result) = list.next() {
            edge_list.insert.push(Edge = result?);
        }
    }
}

//extracts coordinates as tuples from linestring
fn coords(linestring: str) -> Vec<(f64, f64)> {
    let re = Regex::new(r"\((\d+) (\d+)\)").unwrap();
    let coords: Vec<(f64, f64)> = re.captures_iter(linestring).map(|caps| {
        let (_, [lon, lat]) = caps.extract();
        (lon, lat)
    }).collect();
    coords
}

//finds the corners to use for quadtree 
fn quad_corners() -> (f64, f64, f64, f64) {
    let start_lon: (f64) = coords(edge_list.get(0).linestring).get(0).1;
    let start_lat: (f64) = coords(edge_list.get(0).linestring).get(0).1;
    
    let mut max_lon = start_lon;
    let mut max_lat = start_lat;
    let mut min_lon = start_lon;
    let mut min_lat = start_lat
    for edge in edge_list {
        for instance in coords(edge.linestring) {
            if instance.0 > max_lon {
                max_lon = instance.0;
            }
            if instance.1 > max_lat {
                max_lat = instance.1;
            }

            if instance.0 < min_lon {
                min_lon = instance.0;
            }
            if instance.1 < min_lat {
                min_lat = instance.1;
            }
        }
    }
    (max_lon, max_lat, min_lon, min_lat)
}

//takes in a Node and returns the linestring closest to point
fn get_linestring(Node) -> str {
    
}

fn generate_match() -> HashMap<Node, str>{
    let mut map = HashMap::new();
    for node in node_list {
        map.insert(node, get_linestring(node));
    }
    map
}   

//finds subsection where point is located at in linestring
fn get_subsection(Node, linestring: str) -> (f64, f64){
    
}


fn main() {
    println!("start");
    let start = SystemTime::now();

    parse_nodes();
    parse_edges();

    let parsed = SystemTime::now().duration_since(start).expect("Clock may have gone backwards");
    println!("parsed at t = {}", parsed);

    let map = generate_match();

    let matched = SystemTime::now().duration_since(start).expect("Clock may have gone backwards");
    println!("matched at t = {}", matched);

    for (point, linestring) in map {
        println!("point {:?} matches to linestring {:?} at geodesic {:?}", point, linestring, get_subsection(point, linestring));
    }
}