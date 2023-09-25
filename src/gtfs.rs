use std::collections::HashMap;
use gtfs_structures;

fn main() {
    let gfts_rail = gtfs_structures::Gtfs::new("gtfs_rail.zip").unwrap();
    for stop in gfts_rail.stops {
        let data = HashMap::new();
        if stop.1.parent_station == None {
            println!("{} {} {} {}", stop.1.id, stop.1.name, stop.1.longitude.unwrap(), stop.1.latitude.unwrap());
        }
    }
}