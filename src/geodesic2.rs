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

fn point_to_geodesic(mut pA: (f64, f64), pB: (f64, f64), pP: (f64, f64)) -> (f64, f64, f64) {
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
        let (s_ap, ap_azi1, _, m_ap, M_ap, _, _) =
            geod.inverse(pA.0, pA.1, pP.0, pP.1);
        // the 3-tuple gives: azi1, azi2, a12
        let (ab_azi1, _, _) =
            geod.inverse(pA.0, pA.1, pB.0, pB.1);
        let A = ap_azi1 - ab_azi1;
        let mut s_ax: f64 = 0.0;
        if iter_num == 0 {
            s_ax = R * ((s_ap / R).sin() * radians(A).cos()).atan2((s_ap / R).cos());
        } else {
            s_ax = m_ap * radians(A).cos() / ((m_ap / s_ap) * radians(A).cos().exp2() + M_ap * radians(A).sin().exp2());
        }
        let (pA2_lat2, pA2_lon2) = geod.direct(pA.0, pA.1, ab_azi1, s_ax);
        if DEBUG {
            println!("{}, {}, {}, {:.4}", iter_num + 1, dd_to_dms(pA2_lat2), dd_to_dms(pA2_lon2), s_ax)
        }
        if s_ax.abs() < 1e-2 {
            return (pA.0, pA.1, s_ap);
        }
        pA = (pA2_lat2, pA2_lon2);
        iter_num += 1;
    }

}

fn main() {
    point_to_geodesic((52.0, 5.0), (51.4, 6.0), (52.0, 5.5));
}

#[cfg(test)]
#[test]
// tests aren't done :P
fn test_PtG() {
    assert_relative_eq!(1.0, 1.0);
}