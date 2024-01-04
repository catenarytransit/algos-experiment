use std::error::Error;
use csv::Reader;
use osmgraphing::configs::writing::network::graph;
use std::collections::HashMap;
use std::time::SystemTime;
use regex::Regex;

mod graph;

//extracts coordinates as tuples from linestring
fn coords(linestring: str) -> Vec<(f64, f64)> {
    let re = Regex::new(r"\((\d+) (\d+)\)").unwrap();
    let coords: Vec<(f64, f64)> = re.captures_iter(linestring).map(|caps| {
        let (_, [lon, lat]) = caps.extract();
        (lon, lat)
    }).collect();
    coords
}

let mut subsection: (f64, f64) = (None, None);
//takes in a Node and returns the linestring closest to point
fn get_linestring(Node) -> Vec<String> {
    let mut all_coords: Vec<(f64, f64)>; 
    for edge in edge_list {
        all_coords.extend(coords(edge.linestring));
    }
    let init_lon: (f64) = all_coords.get(0).0;
    let init_lat: (f64) = all_coords.get(0).1;
    let mut max_lon = init_lon;
    let mut max_lat = init_lat;
    let mut min_lon = init_lon;
    let mut min_lat = init_lat

    for instance in all_coords {
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
    let mut mid_lon = max_lon - min_lon;
    let mut mid_lat = max_lat - min_lat;
    
    loop {
        break;
    }

    let point = format!("\\({} {}\\)", subsection.0, subsection.1);
    let search = Regex::new(&point).unwrap();
    let mut matches = Vec::new();
    for edge in edge_list {
        for instance in search.find_iter(&edge.linestring) {
            matches.push(capture.as_str().to_string());
        }
    }
    matches
}

fn generate_match() -> HashMap<Node, str>{
    let mut map = HashMap::new();
    //for node in node_list {
    //    map.insert(node, get_linestring(node));
    //}
    map.insert(node_list.get(0), get_linestring(node_list.get(0)));
    map
}   

fn main() {
    println!("start");
    let start = SystemTime::now();

    let graph = Graph::from_csv_par3("edges.csv", "nodes.csv", threads);

    let parsed = SystemTime::now().duration_since(start).expect("Clock may have gone backwards");
    println!("parsed at t = {}", parsed);

    let map = generate_match();

    let matched = SystemTime::now().duration_since(start).expect("Clock may have gone backwards");
    println!("matched at t = {}", matched);

    for (point, linestring) in map {
        println!("point {:?} matches to linestring {:?} at geodesic {:?}", point, linestring, subsection);
    }
}