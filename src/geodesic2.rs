use core::fmt;
// using geographiclib_rs because geographiclib doesnt provide the m12 and M12 required by Karney's improvements to BML
use geographiclib_rs::{Geodesic, InverseGeodesic, DirectGeodesic};
use std::time::SystemTime;

/*
 * primarily a translation of the python code provided in the link below into rust
 * https://sourceforge.net/p/geographiclib/discussion/1026621/thread/21aaff9f/?page=2&limit=25#766f
 */

const DEBUG: bool = true;
// value of semi major axis in WGS84 according to library source code since
// geod.a is a private member
const R: f64 = 6378137.0;
 
#[derive(Debug)]
struct Intercept {
    lat: f64,
    lon: f64,
    dist: f64,
}

struct DMS {
    is_neg: bool,
    deg: u8,
    min: u8,
    sec: f64,
}

impl fmt::Display for DMS {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.is_neg {
            write!(f, "-{}° {}\' {:.4}\"", self.deg, self.min, self.sec)
        } else {
            write!(f, "{}° {}\' {:.4}\"", self.deg, self.min, self.sec)
        }
    }
}

fn dd_to_dms(degs: f64) -> DMS {
    let decimal_deg = degs.abs();
    let decimal_min = (decimal_deg - decimal_deg.floor()) * 60.0;
    let decimal_sec = (decimal_min - decimal_min.floor()) * 60.0;

    DMS {
        is_neg: degs < 0.0,
        deg: decimal_deg as u8,
        min: decimal_min as u8,
        sec: decimal_sec,
    }
}

fn point_to_geodesic(mut p_a: (f64, f64), p_b: (f64, f64), p_p: (f64, f64)) -> Intercept {
    let geod = Geodesic::wgs84();
    let mut iter_num = 0;
    let mut s_ax: f64;
    loop {
        /* 
         * the 7-tuple gives us (in order):
         * s12, azi1, azi2, m12, M12, M21, a12
         * from the library source code (around line 1130 in geodesic.rs as of 
         * f8d9f98), there is no way to get m12 and M12 without a12
         * https://github.com/georust/geographiclib-rs/blob/main/src/geodesic.rs#L1096
         */ 
        let (s_ap, azi1_ap, _, m_ap, mm_ap, _, _) =
            geod.inverse(p_a.0, p_a.1, p_p.0, p_p.1);
        // the 3-tuple gives: azi1, azi2, a12
        let (azi1_ab, _, _) =
            geod.inverse(p_a.0, p_a.1, p_b.0, p_b.1);
        let a = azi1_ap - azi1_ab;
        s_ax = m_ap * a.to_radians().cos() / ((m_ap / s_ap) * a.to_radians().cos().powi(2) + mm_ap * a.to_radians().sin().powi(2));
        if iter_num == 0 {
            s_ax = R * ((s_ap / R).sin() * a.to_radians().cos()).atan2((s_ap / R).cos());
        }
        
        let (p_a2_lat2, p_a2_lon2) = geod.direct(p_a.0, p_a.1, azi1_ab, s_ax);
        if DEBUG {
            eprintln!("{}, {}, {}, {:.4}", iter_num + 1, dd_to_dms(p_a2_lat2), dd_to_dms(p_a2_lon2), s_ax)
        }
        if s_ax.abs() < 1e-2 {
            return Intercept{lat: p_a.0, lon: p_a.1, dist: s_ap};
        }
        p_a = (p_a2_lat2, p_a2_lon2);
        iter_num += 1;
    }
}

fn test_point(p_a: (f64, f64), p_b: (f64, f64), p_p: (f64, f64)) {
    println!("a: ({}, {})     b: ({}, {})     p: ({}, {})", p_a.0,p_a.1,p_b.0,p_b.1,p_p.0,p_p.1);
    let start = SystemTime::now();
    let result: Intercept = point_to_geodesic(p_a, p_b, p_p);
    let end = SystemTime::now();
    let duration = end.duration_since(start).expect("Clock may have gone backwards");
    println!("Result: ({}, {}, {} km) at time {:?}", dd_to_dms(result.lat), dd_to_dms(result.lon), result.dist/1000.0, duration);
}

fn main() {
    println!("24 km case:");
    println!("{:?}", test_point((52.0, 5.0), (51.4, 6.0), (52.0, 5.5)));
    println!("1000 km case:");
    println!("{:?}", test_point((42.0, 29.0), (39.0, -77.0), (64.0, -22.0)));
    println!("12200 km case:");
    println!("{:?}", test_point((42.0, 29.0), (-35.0, -70.0), (64.0, -22.0)));
}

#[cfg(test)]
#[test]
fn test_short() {
    let p_a = (52.0, 5.0);
    let p_b = (51.4, 6.0);
    let p_p = (52.0, 5.5);
    let intercept: Intercept = point_to_geodesic(p_a, p_b, p_p);
    let lat = dd_to_dms(intercept.lat);
    let lon = dd_to_dms(intercept.lon);
    assert!(!lat.is_neg);
    assert_eq!(lat.deg, 51);
    assert_eq!(lat.min, 50);
    assert_relative_eq!(lat.sec, 45.9212, epsilon = 1e-4);
    assert!(!lon.is_neg);
    assert_eq!(lon.deg, 5);
    assert_eq!(lon.min, 15);
    assert_relative_eq!(lon.sec, 37.5426, epsilon = 1e-4);
    eprintln!("calculated distance: {} km", intercept.dist / 1000.0);
}

#[test]
fn test_long() {
    let p_a = (42.0, 29.0);
    let p_b = (39.0, -77.0);
    let p_p = (64.0, -22.0);
    let intercept: Intercept = point_to_geodesic(p_a, p_b, p_p);
    let lat = dd_to_dms(intercept.lat);
    let lon = dd_to_dms(intercept.lon);
    assert!(!lat.is_neg);
    assert_eq!(lat.deg, 54);
    assert_eq!(lat.min, 55);
    assert_relative_eq!(lat.sec, 42.7134, epsilon = 1e-4);
    assert!(lon.is_neg);
    assert_eq!(lon.deg, 21);
    assert_eq!(lon.min, 56);
    assert_relative_eq!(lon.sec, 14.2477, epsilon = 1e-4);
    eprintln!("calculated distance: {} km", intercept.dist / 1000.0);
}

#[test]
fn test_very_long() {
    let p_a = (42.0, 29.0);
    let p_b = (-35.0, -70.0);
    let p_p = (64.0, -22.0);
    let intercept: Intercept = point_to_geodesic(p_a, p_b, p_p);
    let lat = dd_to_dms(intercept.lat);
    let lon = dd_to_dms(intercept.lon);
    assert!(!lat.is_neg);
    assert_eq!(lat.deg, 37);
    assert_eq!(lat.min, 58);
    assert_relative_eq!(lat.sec, 41.2236, epsilon = 1e-4);
    assert!(!lon.is_neg);
    assert_eq!(lon.deg, 18);
    assert_eq!(lon.min, 20);
    assert_relative_eq!(lon.sec, 56.6279, epsilon = 1e-4);
    eprintln!("calculated distance: {} km", intercept.dist / 1000.0);
}
