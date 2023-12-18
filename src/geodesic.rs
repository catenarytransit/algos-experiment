use vincenty::vincenty_direct;

mod vincenty; 
const RADIUS: f64 = 6378137.0;

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
fn point_to_geodesic(p_a: (f64, f64), p_b: (f64, f64), p_p: (f64, f64)) -> (f64, f64) {
    let earth = Ellipsoid::from_descriptor(&algo::WGS84_ELLIPSOID_DESCRIPTOR);  
    loop {
        let ap: (f64, f64, f64, i32, bool) = vincenty_inverse(p_a.0, p_a.1, p_p.0, p_p.1, &earth, 0.5, 32767); //lat then lon
        let ab: (f64, f64, f64, i32, bool) = vincenty_inverse(p_a.0, p_a.1, p_b.0, p_b.1, &earth, 0.5, 32767);
        let s_ap = ap.4; //placeholder
        let a = ap.2 - ab.2;

        let s_px = R * asin(sin(s_ap / R) * sin(radians(A)));
        let s_ax = 2 * R * atan( sin(radians((90.0 + A) / 2.0)) / sin(radians((90.0 - A) / 2.0)) * tan((s_ap - s_px)/(2*R)) );
        
        let p_a2 = vincenty_direct(p_a.0, p_a.1, ab.2, p_p.1);




    }
}
fn main() {
    
}
