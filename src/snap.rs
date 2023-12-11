use std::collections::HashMap;
use chrono::prelude::*;
use gtfs_structures;
#[derive(Debug)]
struct GTFSGraph {
    route_names: HashMap<String, String>,
    stop_names: HashMap<String, String>,
    //<route id, <stop id, <service id, Vec<stop times>>>>
    routes: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
}

impl GTFSGraph {
    fn new() -> Self {
        Self {
            routes: HashMap::new(),
            route_names: HashMap::new(),
            stop_names: HashMap::new(),
        }
    }
    //adding connected edges
    fn add_route(&mut self, id: String, name: String) {
        let stops = HashMap::new();
        self.routes.insert(id.clone(), stops);
        self.route_names.insert(id, name);
    }
    fn add_stop(&mut self, id: String, name: String) {
        self.stop_names.insert(id, name);
    }
    fn add_stoptime(&mut self, id: String, stop_id: String, service_id: String, arrival_time: u32) {//, start_date: &String, end_date: &String) {
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
        if !self.routes.get_mut(&id).unwrap().get_mut(&stop_id).unwrap().contains_key(&service_id) {
            let new_stop_times = vec![arrival_string];
            self.routes.get_mut(&id).unwrap().get_mut(&stop_id).unwrap().insert(service_id, new_stop_times);
        } else {
            self.routes.get_mut(&id).unwrap().get_mut(&stop_id).unwrap().get_mut(&service_id).unwrap().push(arrival_string);
        }
    }
    fn sort_stoptimes(&mut self) {
        for route in &mut self.routes {
            for stop in route.1 {
                for service in stop.1 {
                    service.1.sort();
                }
            }
        }
    }    
}


fn main() {
    let gfts_rail = gtfs_structures::Gtfs::new("gtfs_rail.zip").unwrap();
    let mut graph: GTFSGraph = GTFSGraph::new(); 
    for route in gfts_rail.routes {
        graph.add_route(route.1.id, route.1.long_name);
    }
    for trip in gfts_rail.trips {
        //println!("{}", trip.1.id);
        for stop_times in trip.1.stop_times {
            if !graph.stop_names.contains_key(&stop_times.stop.id) {
                graph.add_stop(stop_times.stop.id.clone(), stop_times.stop.name.clone())
            }
            graph.add_stoptime(trip.1.route_id.clone(), stop_times.stop.id.clone(), trip.1.service_id.clone(), stop_times.arrival_time.unwrap());
        }
    }
    graph.sort_stoptimes();
    println!("{:#?}", graph)
}