use std::collections::HashMap;
use gtfs_structures;

fn main() {
    let gfts_rail = gtfs_structures::Gtfs::new("gtfs_rail.zip").unwrap();
    let mut data: HashMap<String, HashMap<String, String>> = HashMap::new(); // Define the data type correctly

    for route in &gfts_rail.routes {
        let mut hash_route = HashMap::new();
        hash_route.insert(route.1.id.clone(), route.1.long_name.clone());
        data.insert(route.1.id.clone(), hash_route);
    }

    for trip in &gfts_rail.trips {
        let route_id = &trip.1.route_id;
        let shape_id = trip.1.shape_id.clone().unwrap();

        data.entry(route_id.clone()).or_insert_with(HashMap::new).insert(shape_id.clone(), route_id.clone());
    }

    for stop in &gfts_rail.stops {
        if stop.1.parent_station.is_none() {
            let mut station = HashMap::new();
            station.insert("stop_lat".to_string(), stop.1.latitude.unwrap().to_string());
            station.insert("stop_long".to_string(), stop.1.longitude.unwrap().to_string());
            data.insert(stop.1.name.clone(), station);
        }
    }

    for (key, value) in &data {
        println!("{}: {:?}", key, value);
    }
}
