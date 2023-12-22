
use geographiclib_rs::{Geodesic, DirectGeodesic, InverseGeodesic};
// given two points (A, B) and a proportion P,
// find the point X along the geodesic P of the length of the geodesic from A
fn percent_geodesic(a: (f64, f64), b: (f64, f64), p: f64) -> (f64, f64) {
    let geod = Geodesic::wgs84();
    let (s_ab, azi_a, _, _) = geod.inverse(a.0, a.1, b.0, b.1);
    geod.direct(a.0, a.1, azi_a, p * s_ab)
}

fn main() {
    println!("24 km case:");
    println!("{:?}", percent_geodesic((52.0, 5.0), (51.4, 6.0), 0.25));
    println!("1000 km case:");
    println!("{:?}", percent_geodesic((42.0, 29.0), (39.0, -77.0), 0.5));
    println!("12200 km case:");
    println!("{:?}", percent_geodesic((42.0, 29.0), (-35.0, -70.0), 0.75));
}