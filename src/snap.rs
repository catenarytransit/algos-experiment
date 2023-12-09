use std::collections::HashMap;
use gtfs_structures;
#[derive(Debug)]
struct GTFSGraph {
    route_names: HashMap<String, String>,
    stop_names: HashMap<String, String>,
    routes: HashMap<String, HashMap<String, Vec<u32>>>,
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

    fn add_stoptime(&mut self, id: String, stop_id: String, arrival_time: u32) {
        if !self.routes.contains_key(&id) {
            self.add_route(id.clone(), "Kyler's Transit Line".to_string());
        }
        if let Some(stop_times) = self.routes.get_mut(&id).unwrap().get_mut(&stop_id) {
            if !stop_times.contains(&arrival_time) {
                stop_times.push(arrival_time);
            }
        } else {
            let new_stop_times = vec![arrival_time];
            self.routes.get_mut(&id).unwrap().insert(stop_id, new_stop_times);
        }
    }
    fn sort_stoptimes(&mut self) {
        for route in &mut self.routes {
            for stop in route.1 {
                stop.1.sort();
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
            graph.add_stoptime(trip.1.route_id.clone(), stop_times.stop.id.clone(), stop_times.arrival_time.unwrap());
        }
    }
    graph.sort_stoptimes();
    println!("{:#?}", graph)
}