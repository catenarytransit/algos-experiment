
use geographiclib_rs::{Geodesic, DirectGeodesic, InverseGeodesic};
use geo::{coord, Coord, LineString};
use itertools::Itertools;
// given two points (A, B) and a proportion P,
// find the point X along the geodesic P of the length of the geodesic from A
pub fn percent_geodesic(a: (f64, f64), b: (f64, f64), p: f64) -> Coord<f64> {
    let geod = Geodesic::wgs84();
    let (s_ab, azi_a, _, _) = geod.inverse(a.0, a.1, b.0, b.1);
    let (x, y) = geod.direct(a.0, a.1, azi_a, p * s_ab);
    coord! {x: x, y: y,}
}

pub fn percent_linestring(l: LineString, p: f64) -> Coord<f64> {
    let geod = Geodesic::wgs84();
    let mut distances: Vec<f64> = Vec::new();
    for (a, b) in l.0.iter().tuple_windows() {
        distances.push(geod.inverse(a.x, a.y, b.x, b.y));
    }
    let distance: f64 = p * distances.iter().sum::<f64>();
    let mut sum = 0.0;
    let mut i = 0;
    for _ in 0..distances.len() {
        sum += distances[i];
        if sum > distance {
            break;
        }
        i += 1;
    }
    let (azi_a, _, _) = geod.inverse(l[i].x, l[i].y, l[i + 1].x, l[i + 1].y);
    let (x, y) = geod.direct(l[i].x, l[i].y, azi_a, distance - sum + distances[i]);
    coord! {x: x, y: y,}
}

fn main() {
    println!("24 km case:");
    println!("{:?}", percent_geodesic((52.0, 5.0), (51.4, 6.0), 0.25));
    println!("1000 km case:");
    println!("{:?}", percent_geodesic((42.0, 29.0), (39.0, -77.0), 0.5));
    println!("12200 km case:");
    println!("{:?}", percent_geodesic((42.0, 29.0), (-35.0, -70.0), 0.75));
}

#[cfg(test)]
mod tests {
    use super::percent_geodesic;
    use approx::assert_relative_eq;
    #[test]
    fn test_percent_geodesic_from_both_ends_short() {
        let a = (52.0, 5.0);
        let b = (51.4, 6.0);
        assert_relative_eq!(percent_geodesic(a, b, 0.5), percent_geodesic(b, a, 0.5), epsilon=1e-8);
        assert_relative_eq!(percent_geodesic(a, b, 0.75), percent_geodesic(b, a, 0.25), epsilon=1e-8);
        assert_relative_eq!(percent_geodesic(a, b, 0.125), percent_geodesic(b, a, 0.875), epsilon=1e-8);
    }

    #[test]
    fn test_percent_geodesic_from_both_ends_long() {
        let a = (42.0, 29.0);
        let b = (39.0, -77.0);
        assert_relative_eq!(percent_geodesic(a, b, 0.5), percent_geodesic(b, a, 0.5), epsilon=1e-8);
        assert_relative_eq!(percent_geodesic(a, b, 0.75), percent_geodesic(b, a, 0.25), epsilon=1e-8);
        assert_relative_eq!(percent_geodesic(a, b, 0.125), percent_geodesic(b, a, 0.875), epsilon=1e-8);
    }

    #[test]
    fn test_percent_geodesic_from_both_ends_very_long() {
        let a = (42.0, 29.0);
        let b = (-35.0, -70.0);
        assert_relative_eq!(percent_geodesic(a, b, 0.5), percent_geodesic(b, a, 0.5), epsilon=1e-8);
        assert_relative_eq!(percent_geodesic(a, b, 0.75), percent_geodesic(b, a, 0.25), epsilon=1e-8);
        assert_relative_eq!(percent_geodesic(a, b, 0.125), percent_geodesic(b, a, 0.875), epsilon=1e-8);
    }
}