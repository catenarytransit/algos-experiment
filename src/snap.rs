use gtfs_structures;
fn main() {
    let gfts_rail = gtfs_structures::Gtfs::new("gtfs_rail.zip").unwrap();
    for stop in gfts_rail.stops {
        if stop.parent_station.is_none() {
            println!("{} {}", stop.id, stop.name );
        }
    }
}