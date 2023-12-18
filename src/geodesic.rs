use vincenty::*;
use std::f64::consts::PI;

mod vincenty; 
const R: f64 = 6378137.0;

fn radians(degs: f64) -> f64 {
    return degs * PI / 180.0;
}

fn dd_to_dms(degs: f64) -> (bool, u32, u32, f64) {
    let is_negative = degs < 0.0;
    let degs_abs = degs.abs();

    let d_int = degs_abs.floor() as u32;
    let remainder_minutes = (degs_abs - f64::from(d_int)) * 60.0;
    let m_int = remainder_minutes.floor() as u32;
    let secs = (remainder_minutes - f64::from(m_int)) * 60.0;

    (is_negative, d_int, m_int, secs)
}

fn dms_str(val: f64) -> String {
    let dms = dd_to_dms(val);
    format!("{}{}° {}' {:.4}\"", if dms.0 { "-" } else { "" }, dms.1, dms.2, dms.3)
}

static DEBUG: bool = false;
fn point_to_geodesic(mut p_a: (f64, f64), p_b: (f64, f64), p_p: (f64, f64)) -> (f64, f64) {
    let earth = Ellipsoid::from_descriptor(&vincenty::WGS84_ELLIPSOID_DESCRIPTOR);  
    loop {
        //pub fn vincenty_inverse(sp_lat_rad: f64, sp_lon_rad: f64, ep_lat_rad: f64, ep_lon_rad: f64, el: &Ellipsoid, eps: f64, it_limit: i32) -> (dst_m, wrap_2pi(fwd_az_rad), wrap_2pi(rev_az_rad), its, ((its < it_limit) && (it_check < consts::PI)))
        let ap: (f64, f64, f64, i32, bool) = vincenty_inverse(p_a.0, p_a.1, p_p.0, p_p.1, &earth, 0.5, 32767); //lat then lon
        let ab: (f64, f64, f64, i32, bool) = vincenty_inverse(p_a.0, p_a.1, p_b.0, p_b.1, &earth, 0.5, 32767);
       
        //let s_ap = ap.1 as f64; 
        //let a = ap.2 - ab.2;
        //let s_px: f64 = R * (s_ap / R).sin().asin() * radians(a).sin();
        //let s_ax: f64 = 2.0 * R * ((radians((90.0 + a) / 2.0)).sin() / (radians((90.0 - a) / 2.0)).sin() * ((s_ap - s_px)/(2.0 * R)).tan()).atan();
        
        /*python code calculautes s_ax as the distance from the first point to the second point in meters, 
          but rust vincenty_inverse returns ellipsoidal distance between the two points and takes in said value for vincenty_direct
          thus, we do not need the above calcuations (i think)
        */
        //pub fn vincenty_direct(sp_lat_rad: f64, sp_lon_rad: f64, fwd_az_rad: f64, dst_m: f64, el: &Ellipsoid, eps: f64, it_limit: i32) ->  (wrap_2pi(ep_lat_rad), wrap_2pi(ep_lon_rad), wrap_2pi(rev_az_rad), its)
        let p_a2 = vincenty_direct(p_a.0, p_a.1, ab.1, ap.0,  &earth, 0.5, 32767); //placeholder


        if DEBUG {
            println!("lat2: {}   lon2: {}    distance: {}", p_a2.0, p_a2.1, ap.0);
        }

        if ap.0.abs() < 1e-8 {
            break;
        }
        
        p_a = (p_a2.0, p_a2.1)
    }
}

fn Test(mut p_a: (f64, f64), p_b: (f64, f64), p_p: (f64, f64)) {
    println!("a: ({},{})     b: ({},{})     p: ({},{})", p_a.0,p_a.1,p_b.0,p_b.1,p_p.0,p_p.1);
    let result: (f64,f64) = point_to_geodesic(p_a, p_b, p_p);
    println!("Result: ({},{})", result.0, result.1);
}

fn main() {
    println!("start");
    Test((52.0, 5.0), (51.4, 6.0), (52.0, 5.5));
    Test((42.0, 29.0), (39.0, -77.0), (64.0, -22.0));
    Test((42.0, 29.0), (-35.0, -70.0), (64.0, -22.0));
    println!("done");
}
