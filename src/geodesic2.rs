use core::fmt;
use std::f64::consts::PI;
use approx::assert_relative_eq;
// using geographiclib_rs because geographiclib doesnt provide the m12 and M12 required by Karney's improvements to BML
use geographiclib_rs::{Geodesic, InverseGeodesic, DirectGeodesic, capability};


/*
 * primarily a translation of the python code provided in the link below into rust
 * https://sourceforge.net/p/geographiclib/discussion/1026621/thread/21aaff9f/?page=2&limit=25#766f
 */

 const DEBUG: bool = true;
 const OUTMASK: u64 =
    capability::STANDARD | capability::REDUCEDLENGTH | capability::GEODESICSCALE;

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

fn radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn point_to_geodesic(mut p_a: (f64, f64), p_b: (f64, f64), p_p: (f64, f64)) -> Intercept {
    let geod = Geodesic::wgs84();
    // value of semi major axis in WGS84 according to library source code since
    // geod.a is a private member
    let R: f64 = 6378137.0;
    let mut iter_num = 0;
    loop {
        /* 
         * the 7-tuple gives us (in order):
         * s12, azi1, azi2, m12, M12, M21, a12
         * from the library source code (around line 1130 in geodesic.rs as of
         * f8d9f98), there is no way to get m12 and M12 without a12
         * i don't know enough rust to know if there's a better way than typing out the full 7-tuple
         */ 
        let (s_ap, azi1_ap, _, m_ap, M_ap, _, _) =
            geod.inverse(p_a.0, p_a.1, p_p.0, p_p.1);
        // the 3-tuple gives: azi1, azi2, a12
        let (azi1_ab, _, _) =
            geod.inverse(p_a.0, p_a.1, p_b.0, p_b.1);
        let A = azi1_ap - azi1_ab;
        let mut s_ax: f64 = 0.0;
        if iter_num == 0 {
            s_ax = R * ((s_ap / R).sin() * radians(A).cos()).atan2((s_ap / R).cos());
        } else {
            s_ax = m_ap * radians(A).cos() / ((m_ap / s_ap) * radians(A).cos().exp2() + M_ap * radians(A).sin().exp2());
        }
        let (p_a2_lat2, p_a2_lon2) = geod.direct(p_a.0, p_a.1, azi1_ab, s_ax);
        if DEBUG {
            println!("{}, {}, {}, {:.4}", iter_num + 1, dd_to_dms(p_a2_lat2), dd_to_dms(p_a2_lon2), s_ax)
        }
        if s_ax.abs() < 1e-2 {
            return Intercept{lat: p_a.0, lon: p_a.1, dist: s_ap};
        }
        p_a = (p_a2_lat2, p_a2_lon2);
        iter_num += 1;
    }

}

fn main() {
    point_to_geodesic((52.0, 5.0), (51.4, 6.0), (52.0, 5.5));
}

#[cfg(test)]
#[test]
fn test_short() {
    let p_a = (52.0, 5.0);
    let p_b = (51.4, 6.0);
    let p_p = (52.0, 5.5);
    let intercept: Intercept = point_to_geodesic(p_a, p_b, p_p);
    assert_relative_eq!(intercept.lat, 51.8460892222, epsilon = 1e-6);
    assert_relative_eq!(intercept.lon, 5.2604285, epsilon = 1e-6);
    eprintln!("expected distance:  ~24 km");
    eprintln!("calculated distance: {} km", intercept.dist / 1000.0);
}

#[test]
fn test_long() {
    let p_a = (42.0, 29.0);
    let p_b = (39.0, -77.0);
    let p_p = (64.0, -22.0);
    let intercept: Intercept = point_to_geodesic(p_a, p_b, p_p);
    assert_relative_eq!(intercept.lat, 54.9285315, epsilon = 1e-6);
    assert_relative_eq!(intercept.lon, -21.9372910278, epsilon = 1e-6);
    eprintln!("expected distance:  ~1000 km");
    eprintln!("calculated distance: {} km", intercept.dist / 1000.0);
}

#[test]
fn test_very_long() {
    let p_a = (42.0, 29.0);
    let p_b = (-35.0, -70.0);
    let p_p = (64.0, -22.0);
    let intercept: Intercept = point_to_geodesic(p_a, p_b, p_p);
    assert_relative_eq!(intercept.lat, 37.9781176667, epsilon = 1e-6);
    assert_relative_eq!(intercept.lon, 18.3490633056, epsilon = 1e-6);
    eprintln!("expected distance:  ~12200 km");
    eprintln!("calculated distance: {} km", intercept.dist / 1000.0);

}