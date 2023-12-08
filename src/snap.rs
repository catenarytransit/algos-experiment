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
    fn add_route(&mut self, id: String, name: String) {
        let stops: HashMap<String, Vec<u32>> = HashMap::new();
        self.routes.insert(id.clone(), stops);
        self.route_names.insert(id, name);
    }
    fn add_stop(&mut self, id: String, name: String) {
        self.stop_names.insert(id, name);
    }
    fn add_stoptime(&mut self, id: String, stop_id: String, arrival_time: u32) {
        if let Some(route_stops) = self.routes.get_mut(&id) {
            if let Some(stop_times) = route_stops.get_mut(&stop_id) {
                stop_times.push(arrival_time);
            } else {
                let new_stop_times = vec![arrival_time];
                route_stops.insert(stop_id, new_stop_times);
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
            graph.add_stoptime(trip.1.id.clone(), stop_times.stop.id.clone(), stop_times.arrival_time.unwrap());
        }
    }
    println!("{:#?}", graph)
}