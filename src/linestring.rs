mod graph;
use graph::Node;
use std::collections::HashMap;
use std::time::SystemTime;
use regex::Regex;

use graph::Graph;

//extracts coordinates as tuples from linestring
fn coords(linestring: &str) -> Vec<(f64, f64)> {
    let re = Regex::new(r"\((\d+) (\d+)\)").unwrap();
    let coords: Vec<(f64, f64)> = re.captures_iter(&linestring).map(|caps| {
        let (_, [lon, lat]) = caps.extract();
        (lon.parse::<f64>().unwrap(), lat.parse::<f64>().unwrap())
    }).collect();
    coords
}

//let mut subsection: (f64, f64) = (None, None);
//takes in a Node and returns the linestring closest to point
fn get_linestring(node: Node) -> Vec<String> {
    //let mut all_coords: Vec<(f64, f64)>;
    let mut all_coords: Vec<(f64, f64)> = vec![(31.0, 128.0)];
    //for edge in edge_list {
        //all_coords.extend(coords(edge.linestring));
    //}
    let init_lon: f64 = all_coords.get(0).unwrap().0;
    let init_lat: f64 = all_coords.get(0).unwrap().1;
    let mut max_lon = init_lon;
    let mut max_lat = init_lat;
    let mut min_lon = init_lon;
    let mut min_lat = init_lat;

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

    //let point = format!("\\({} {}\\)", subsection.0, subsection.1);
    //let search = Regex::new(&point).unwrap();
    let mut matches = Vec::new();
    /*for edge in edge_list {
        for instance in search.find_iter(&edge.linestring) {
            matches.push(capture.as_str().to_string());
        }
    }*/
    matches
}

fn generate_match() -> HashMap<Node, String>{
    let mut map = HashMap::new();
    //for node in node_list {
    //    map.insert(node, get_linestring(node));
    //}
    //map.insert(node_list.get(0), get_linestring(node_list.get(0)));
    map
}   

fn main() {
    println!("start");
    let start = SystemTime::now();

    let graph = Graph::from_csv_par3("../testedges.csv", "../testnodes.csv", 32);

    let mut sort_x = graph.nodes.clone();
    sort_x.sort_by(|a, b| a.lon.partial_cmp(&b.lon).unwrap());
    let mut sort_y = graph.nodes.clone();
    sort_y.sort_by(|a, b| a.lat.partial_cmp(&b.lat).unwrap());
    
    let parsed = SystemTime::now().duration_since(start).expect("Clock may have gone backwards");
    println!("parsed at t = {:?}", parsed);
    let iter_x = sort_x.iter();
    
    println!("aaa {}", 	iter_x.len());
    
    for node in iter_x {
    	println!("point {:?}", node);
    }
    
    
    
    
    //for node in y_iter() {
    //	println!("point {:?}", node);
    //}
    //let map = generate_match();

    //let matched = SystemTime::now().duration_since(start).expect("Clock may have gone backwards");
    //println!("matched at t = {:?}", matched);

    //for (point, linestring) in map {
        //println!("point {:?} matches to linestring {:?} at geodesic {:?}", point, linestring, subsection);
    //}
}
