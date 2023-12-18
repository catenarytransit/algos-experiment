mod lib; 
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
fn point_to_geodesic(pA: (f64, f64), pB: (f64, f64), pP: (f64, f64)) -> (f64, f64) {
    (0.0, 0.0)
}
fn main() {
    
}
