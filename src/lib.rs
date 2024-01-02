use std::{fs::File, collections::HashMap, thread, sync::{Arc, Mutex}};

use csv::{ReaderBuilder, StringRecord};
use gtfs_structures::DirectionType;
use serde::{Serialize, Deserialize};
use tokio_postgres::Client;

 
#[derive(Serialize, Deserialize, Debug)]
struct GTFSGraph {
    onestop_id: String,
    old_services: Vec<String>,
    route_names: HashMap<String, String>,
    stop_names: HashMap<String, String>,
    //<route id, <stop id, <service id, Vec<stop times>>>>
    routes: HashMap<String, HashMap<String, HashMap<String, Vec<Vec<String>>>>>,
}

impl GTFSGraph {
    fn new(onestop_id: &str) -> Self {
        Self {
            onestop_id: onestop_id.to_string(),
            old_services: Vec::new(),
            routes: HashMap::new(),
            route_names: HashMap::new(),
            stop_names: HashMap::new(),
        }
    }
    async fn to_sql(&mut self, client: &Client) {
        for (route, stops) in &self.routes {
            // Iterate over the middle HashMap
            for (stop, services) in stops {
                // Iterate over the innermost HashMap
                for (service, times) in services {
                    let timetable = serde_json::to_string(&times.last().unwrap());
                    let trips = serde_json::to_string(&times.first().unwrap());
                    // Prepare the SQL statement with parameterized query
                    let service_id: String = service[0..service.len() - 2].to_string();
                    let direction: String = service[service.len() - 1..].to_string();
                    let statement = "INSERT INTO timetable(id, onestop_id, route, stop, service, direction, trip_id, time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *";
                    let _ = client.query_one(statement, &[&format!("{}-{}-{}-{}", self.onestop_id, route, stop, service), &self.onestop_id, &route, &stop, &service_id,  &direction, &trips.unwrap(), &timetable.unwrap()]).await;
                    //println!("{:#?}", rows)
                }
            }
        }
    }
    async fn from_sql(&mut self, client: &Client) {

    }
    //adding connected edges
    fn add_route(&mut self, id: String, name: String) {
        let stops = HashMap::new();
        self.routes.insert(id.clone(), stops);
        self.route_names.insert(id, name);
    }
    fn exclude_service(&mut self, id: String) {
        self.old_services.push(id);
    }
    fn add_stop(&mut self, id: String, name: String) {
        self.stop_names.insert(id, name);
    }
    fn add_stoptime(&mut self, id: String, stop_id: String, service_id: String, arrival_time: u32, direction_id: DirectionType, trip_id: String) {//, start_date: &String, end_date: &String) {
        if self.old_services.contains(&service_id) {
            return;
        }
        if !self.routes.contains_key(&id) {
            self.add_route(id.clone(), "Kyler's Transit Line".to_string());
        }
        let mut arrival_string = (arrival_time/3600).to_string();
        if arrival_string.len() == 1 {
            arrival_string.insert_str(0, "0");
        }
        if ((arrival_time % 3600)/60).to_string().as_str().len() == 1 {
            arrival_string.push_str(format!(":0{}", ((arrival_time % 3600)/60).to_string()).as_str());
        } else {
            arrival_string.push_str(format!(":{}", ((arrival_time % 3600)/60).to_string()).as_str());
        }
        if !self.routes.get_mut(&id).unwrap().contains_key(&stop_id) {
            self.routes.get_mut(&id).unwrap().insert(stop_id.clone(), HashMap::new());
        }
        let mut direction = format!("{:?}", direction_id);
        if direction == "Outbound" {
            direction = 0.to_string();
        } else {
            direction = 1.to_string();
        }
        if !self.routes.get_mut(&id).unwrap().get_mut(&stop_id).unwrap().contains_key(&format!("{}-{}", service_id, direction)) {
            let new_stop_times = vec![vec![trip_id], vec![arrival_string]];
            self.routes.get_mut(&id).unwrap().get_mut(&stop_id).unwrap().insert(format!("{}-{}", service_id, direction), new_stop_times);
        } else {
            self.routes.get_mut(&id).unwrap().get_mut(&stop_id).unwrap().get_mut(&format!("{}-{}", service_id, direction)).unwrap()[0].push(trip_id);
            self.routes.get_mut(&id).unwrap().get_mut(&stop_id).unwrap().get_mut(&format!("{}-{}", service_id, direction)).unwrap()[1].push(arrival_string);
        }
    }
    fn clean(&mut self) {
        for route in &mut self.routes {
            for stop in route.1 {
                for service in stop.1 {
                    for stuff in service.1 {
                        stuff.sort();
                    }
                }
            }
        }
    }
}


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

    fn from_csv_par3(edge_file_path: &str, node_file_path: &str, threads: u32) -> Self {
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
        let records: Vec<StringRecord> = ReaderBuilder::new().from_reader(edges).records().collect::<Result<_, _>>().unwrap();
        let records_per_part = records.len() / threads as usize;
        let mut split_records: Vec<_> = Vec::new();

        for i in 0..threads {
            let start_idx = (i * records_per_part as u32) as usize;
            let end_idx = ((i + 1) * records_per_part as u32) as usize;
            if end_idx <= records.len() {
                split_records.push(records[start_idx..end_idx].to_vec());
            } else {
                split_records.push(records[start_idx..records.len()].to_vec());
            }
        }

        let edges: Vec<_> = split_records.into_iter().filter_map(|chunk| Some({ 
            thread::spawn(move || {
                let mut edges: Vec<Edge> = Vec::new();
                for record in chunk {
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
                    edges.push(edge);
                }
                edges
            })
        })).collect();

        for chunk in edges {
            for edge in chunk.join().unwrap() {
                graph.add_edge_obj(edge);
            }
        }

        let nodes = File::open(node_file_path).unwrap();
        let records: Vec<StringRecord> = ReaderBuilder::new().from_reader(nodes).records().collect::<Result<_, _>>().unwrap();
        // At this point, rdr is still in scope, so records can be collected before it's dropped.
        let records_per_part = records.len() / threads as usize;
        let mut split_records: Vec<_> = Vec::new();

        for i in 0..threads {
            let start_idx = (i * records_per_part as u32) as usize;
            let end_idx = ((i + 1) * records_per_part as u32) as usize;
            if end_idx <= records.len() {
                split_records.push(records[start_idx..end_idx].to_vec());
            } else {
                split_records.push(records[start_idx..records.len()].to_vec());
            }
        }

        let nodes: Vec<_> = split_records.into_iter().filter_map(|chunk| Some({ 
            thread::spawn(move || {
                let mut nodes: Vec<Node> = Vec::new();
                for record in chunk {
                    let node = Node {
                        id: record[0].parse().unwrap(),
                        lon: record[1].parse().unwrap(),
                        lat: record[2].parse().unwrap()
                    };
                    nodes.push(node);
                }
                nodes
            })
        })).collect();
        for chunk in nodes {
            for node in chunk.join().unwrap() {
                graph.add_node_obj(node);
            }
        }
        graph
    }

    fn from_csv_par4(edge_file_path: &str, node_file_path: &str, threads: u32) -> Self {
        let graph = Arc::new(Mutex::new(Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }));
        let edges = File::open(edge_file_path).unwrap();
        let records: Vec<StringRecord> = ReaderBuilder::new().from_reader(edges).records().collect::<Result<_, _>>().unwrap();
        let records_per_part = records.len() / threads as usize;
        let mut split_records = Vec::new();

        for i in 0..threads {
            let start_idx = (i * records_per_part as u32) as usize;
            let end_idx = ((i + 1) * records_per_part as u32) as usize;
            if end_idx <= records.len() {
                split_records.push(records[start_idx..end_idx].to_vec());
            } else {
                split_records.push(records[start_idx..records.len()].to_vec());
            }
        }

        let handles: Vec<_> = split_records.into_iter().filter_map(|chunk| Some({ 
            thread::spawn({
                let shared_graph_clone = Arc::clone(&graph);
                move || {
                    let mut edges: Vec<Edge> = Vec::new();
                    for record in chunk {
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
                        edges.push(edge);
                    }
                    let mut graph = shared_graph_clone.lock().unwrap();
                    for edge in edges {
                        graph.add_edge_obj(edge);
                    }
                }
            })
        })).collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let nodes = File::open(node_file_path).unwrap();
        let records: Vec<StringRecord> = ReaderBuilder::new().from_reader(nodes).records().collect::<Result<_, _>>().unwrap();
        let records_per_part = records.len() / threads as usize;
        let mut split_records: Vec<_> = Vec::new();

        for i in 0..threads {
            let start_idx = (i * records_per_part as u32) as usize;
            let end_idx = ((i + 1) * records_per_part as u32) as usize;
            if end_idx <= records.len() {
                split_records.push(records[start_idx..end_idx].to_vec());
            } else {
                split_records.push(records[start_idx..records.len()].to_vec());
            }
        }

        let handles: Vec<_> = split_records.into_iter().filter_map(|chunk| Some({ 
            thread::spawn({
                let shared_graph_clone = Arc::clone(&graph);
                move || {
                    let mut nodes: Vec<Node> = Vec::new();
                    for record in chunk {
                        let node = Node {
                            id: record[0].parse().unwrap(),
                            lon: record[1].parse().unwrap(),
                            lat: record[2].parse().unwrap()
                        };
                        nodes.push(node);
                    }
                    let mut graph = shared_graph_clone.lock().unwrap();
                    for node in nodes {
                        graph.add_node_obj(node);
                    }
                }
            })
        })).collect();

        for handle in handles {
            handle.join().unwrap();
        }
        
        return graph.lock().unwrap().to_owned();
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