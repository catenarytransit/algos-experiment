use std::f64;
use std::f64::consts;

const PI2                   : f64 = 2.0 * consts::PI;
const PI_DBY_180            : f64 = consts::PI / 180.0;
const D180_DBY_PI           : f64 = 180.0 / consts::PI;

/// Default iterations limit for Vincenty's formulas evaluation
pub const VNC_DEF_IT_LIMIT  : i32 = 2000;

/// Default precision threhsold for Vincenty's formulas evaluation
pub const VNC_DEF_EPSILON   : f64 = 1E-12;

const NLM_A                 : f64 = 1.0;
const NLM_B                 : f64 = 0.5;
const NLM_R                 : f64 = 0.5;
const NLM_Q                 : f64 = 0.5;
const NLM_G                 : f64 = 2.0;

/// Default iterations limit for Nelder-Mead optimization algorithm
pub const NLM_DEF_IT_LIMIT  : i32 = 1200;

/// Default precision threhsold for Nelder-Mead optimization algorithm
pub const NLM_DEF_PREC_THRLD: f64 = 1E-12;

/// Default iterations limit for Newton-Gauss optimization algorithm
pub const AOA_NG_DEF_IT_LIMIT: i32 = 1000;
pub const AOA_NG_DEF_PREC_THRLD: f64 = 1E-12;

/// Structure to store two main ellipsoid parameters: Major semi-axis and inverse flattening
pub struct EllipsoidDescriptor {
    /// Major semi-axis
    mjsa_m: f64,
    /// Inverse flattening
    ifltn: f64,
}

/// Structure to store extended list of an ellipsoid parameters 
pub struct Ellipsoid {
    /// Major semi-axis
    mjsa_m: f64,
    /// Inverse flattening
    ifltn: f64,
    /// Flattening
    fltn: f64,
    /// Minor semi-axis
    mnsa_m: f64,
    /// Eccenticity
    ecnt: f64,    
    /// Eccentricity squared
    ecnt_sq: f64,
} 

impl Ellipsoid {    
    /// Builds an Ellipsoid structure from major semi-axis and inverse flattening values
    pub fn new(mjsa_m: f64, ifltn: f64) -> Ellipsoid {
        
        if mjsa_m <= 0.0 {
            panic!("Specified major semi-axis 'mjsa_m' should be greater than zero");
        }
        if ifltn <= 0.0 {
            panic!("Specifed inverse flattening 'ifltn' should be greater than zero")
        }        

        let fltn = 1.0 / ifltn;
        let mnsa_m = mjsa_m * (1.0 - fltn);
        let ecnt_sq = (mjsa_m.powi(2) - mnsa_m.powi(2)) / mjsa_m.powi(2);
        let ecnt = ecnt_sq.sqrt();
    
        Ellipsoid {
            mjsa_m,
            ifltn,
            fltn,
            mnsa_m,
            ecnt,
            ecnt_sq,
        }
    }
    /// Builds an Ellipsoid structure from an EllipsoidDescriptor structure
    pub fn from_descriptor(ed: &EllipsoidDescriptor) -> Ellipsoid {
        Ellipsoid::new(ed.mjsa_m, ed.ifltn)        
    }
}

pub fn wrap(value: f64, bound: f64) -> f64 {
    if bound <= 0.0 {
        panic!("Specified 'bound' value should be greater than zero");
    }

    let mut vl = value.abs();
    let sign = value.signum();

    while vl > bound {
        vl -= bound;
    }

    (vl * sign)
}

pub fn wrap_2pi(value: f64) -> f64 {
    wrap(value, PI2)
}

/// Residual function for Nelder-Mead (simplex) optimizer
pub type Eps3dFunc<T> = fn(&[T], f64, f64, f64) -> f64;

pub const WGS84_ELLIPSOID_DESCRIPTOR: EllipsoidDescriptor = EllipsoidDescriptor{ mjsa_m: 6378137.0, ifltn: 298.257223563 };

pub fn vincenty_inverse(sp_lat_rad: f64, sp_lon_rad: f64, ep_lat_rad: f64, ep_lon_rad: f64, el: &Ellipsoid, eps: f64, it_limit: i32) -> (f64, f64, f64, i32, bool) {
    
    let l_ = ep_lon_rad - sp_lon_rad;
    let tan_u_1 = (1.0 - el.fltn) * sp_lat_rad.tan();
    let cos_u_1 = 1.0 / (1.0 + tan_u_1.powi(2)).sqrt();
    let sin_u_1 = tan_u_1 * cos_u_1;

    let tan_u_2 = (1.0 - el.fltn) * ep_lat_rad.tan();
    let cos_u_2 = 1.0 / (1.0 + tan_u_2 * tan_u_2).sqrt();
    let sin_u_2 = tan_u_2 * cos_u_2;
            
    let mut sin_lambda;
    let mut cos_lambda;
    let mut sin_sigma = 0.0;
    let mut cos_sigma = 0.0;
    let mut sin_alpha;
    let mut sin_sq_sigma;
    let mut cos_sq_alpha = 0.0;
    let mut cos_2_sigma_m = 0.0;
    let mut sigma = 0.0;
    let mut c_;

    let mut lambda = l_;
    let mut lambda_ = 0.0;
    let mut its = 0;

    let mut it_check = 0.0;    
    let antimeridian : bool = l_.abs() > consts::PI;

    while {

        sin_lambda = lambda.sin();
        cos_lambda = lambda.cos();

        sin_sq_sigma = (cos_u_2 * sin_lambda) * (cos_u_2 * sin_lambda) +
                       (cos_u_1 * sin_u_2 - sin_u_1 * cos_u_2 * cos_lambda) * (cos_u_1 * sin_u_2 - sin_u_1 * cos_u_2 * cos_lambda);

        if sin_sq_sigma.abs() > f64::EPSILON
        {
            sin_sigma = sin_sq_sigma.sqrt();
            cos_sigma = sin_u_1 * sin_u_2 + cos_u_1 * cos_u_2 * cos_lambda;
            sigma = sin_sigma.atan2(cos_sigma);
            sin_alpha = cos_u_1 * cos_u_2 * sin_lambda / sin_sigma;

            cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;
            
            if cos_sq_alpha != 0.0 {
                cos_2_sigma_m = cos_sigma - 2.0 * sin_u_1 * sin_u_2 / cos_sq_alpha;
            }
            else {
                cos_2_sigma_m = 0.0;
            }

            c_ = el.fltn / 16.0 * cos_sq_alpha * (4.0 + el.fltn * (4.0 - 3.0 * cos_sq_alpha));
            lambda_ = lambda;
            lambda = l_ + (1.0 - c_) * el.fltn * sin_alpha *
                     (sigma + c_ * sin_sigma * (cos_2_sigma_m + c_ * cos_sigma * (-1.0 + 2.0 * cos_2_sigma_m * cos_2_sigma_m)));
        
            if antimeridian {
                it_check = lambda.abs() - consts::PI;
            }
            else
            {
                it_check = lambda.abs();
            }        
        }
            
        its += 1;
        (((lambda - lambda_).abs() > eps) && (its < it_limit) && (it_check < consts::PI))
    } { }


    let u_sq = cos_sq_alpha * (el.mjsa_m.powi(2) - el.mnsa_m.powi(2)) / el.mnsa_m.powi(2);
    let a_ = 1.0 + u_sq/16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let b_ = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));
    let delta_sigma = b_ * sin_sigma * (cos_2_sigma_m + b_/4.0 * (cos_sigma * (-1.0 + 2.0 * cos_2_sigma_m * cos_2_sigma_m) -
        b_/6.0 * cos_2_sigma_m * (-3.0 + 4.0 * sin_sigma * sin_sigma) * (-3.0 + 4.0 * cos_2_sigma_m * cos_2_sigma_m)));

    let dst_m = el.mnsa_m * a_ * (sigma - delta_sigma);

    let fwd_az_rad = (cos_u_2 * sin_lambda).atan2(cos_u_1 * sin_u_2 - sin_u_1 * cos_u_2 * cos_lambda);
    let rev_az_rad = (cos_u_1 * sin_lambda).atan2(- sin_u_1 * cos_u_2 + cos_u_1 * sin_u_2 * cos_lambda);

    (dst_m, wrap_2pi(fwd_az_rad), wrap_2pi(rev_az_rad), its, ((its < it_limit) && (it_check < consts::PI)))
}

pub fn vincenty_direct(sp_lat_rad: f64, sp_lon_rad: f64, fwd_az_rad: f64, dst_m: f64, el: &Ellipsoid, eps: f64, it_limit: i32) -> (f64, f64, f64, i32) {

    let sin_alpha_1 = fwd_az_rad.sin();
    let cos_alpha_1 = fwd_az_rad.cos();
    let tan_u_1 = (1.0 - el.fltn) * sp_lat_rad.tan();
    let cos_u_1 = 1.0 / (1.0 + tan_u_1 * tan_u_1).sqrt();
    let sin_u_1 = tan_u_1 * cos_u_1;

    let sigma_1 = tan_u_1.atan2(cos_alpha_1);
    let sin_alpha = cos_u_1 * sin_alpha_1;
    let cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;
    let u_sq = cos_sq_alpha * (el.mjsa_m.powi(2) - el.mnsa_m.powi(2)) / el.mnsa_m.powi(2);
    let a_ = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let b_ = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));

    let mut cos_2_sigma_m;
    let mut sin_sigma;
    let mut cos_sigma;
    let mut delta_sigma;

    let mut sigma = dst_m / (el.mnsa_m * a_);
    let mut sigma_;
    let mut its = 0;
            
    while {
        cos_2_sigma_m = (2.0 * sigma_1 + sigma).cos();
        sin_sigma = sigma.sin();
        cos_sigma = sigma.cos();

        delta_sigma = b_ * sin_sigma * (cos_2_sigma_m + b_ / 4.0 * (cos_sigma * (-1.0 + 2.0 * cos_2_sigma_m * cos_2_sigma_m) -
                      b_ / 6.0 * cos_2_sigma_m * (-3.0 + 4.0 * sin_sigma * sin_sigma) * (-3.0 + 4.0 * cos_2_sigma_m * cos_2_sigma_m)));

        sigma_ = sigma;
        sigma = dst_m / (el.mnsa_m * a_) + delta_sigma;

        its += 1;
        (((sigma - sigma_).abs() > eps) && (its < it_limit))
    } { }
    
    let x = sin_u_1 * sin_sigma - cos_u_1 * cos_sigma * cos_alpha_1;
    let ep_lat_rad = (sin_u_1 * cos_sigma + cos_u_1 * sin_sigma * cos_alpha_1).atan2((1.0 - el.fltn) * (sin_alpha * sin_alpha + x * x).sqrt());
    
    let lambda = (sin_sigma * sin_alpha_1).atan2(cos_u_1 * cos_sigma - sin_u_1 * sin_sigma * cos_alpha_1);
    let c_ = el.fltn / 16.0 * cos_sq_alpha * (4.0 + el.fltn * (4.0 - 3.0 * cos_sq_alpha));
    
    let l_ = lambda - (1.0 - c_) * el.fltn * sin_alpha * (sigma + c_ * sin_sigma * 
             (cos_2_sigma_m + c_ * cos_sigma * (-1.0 + 2.0 * cos_2_sigma_m * cos_2_sigma_m)));
    
    let ep_lon_rad = sp_lon_rad + l_;
    let rev_az_rad = sin_alpha.atan2(-x);
    (wrap_2pi(ep_lat_rad), wrap_2pi(ep_lon_rad), wrap_2pi(rev_az_rad), its)
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        assert!(
            (*a - *b).abs() < eps,
            "assertion failed \
             (left: `{:?}`, right: `{:?}`, eps: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            (*a - *b).abs()
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate rand;
    use rand::{Rng, thread_rng};

    #[test]
    fn test_const_ellipsoid_descriptors() {
        assert_approx_eq!(WGS84_ELLIPSOID_DESCRIPTOR.mjsa_m, 6378137.0, 1E-6);
        assert_approx_eq!(WGS84_ELLIPSOID_DESCRIPTOR.ifltn, 298.257223563, 1E-6);   
    }    

    #[test]
    #[should_panic(expected = "Specified major semi-axis 'mjsa_m' should be greater than zero")]
    fn test_ellipsoid_new_non_positive_major_semiaxis_panic() {
        let _el: Ellipsoid = Ellipsoid::new(-1.0, 0.0);
    }

    #[test]
    #[should_panic(expected = "Specifed inverse flattening 'ifltn' should be greater than zero")]
    fn test_ellipsoid_new_inverse_flattening_out_of_range_panic() {
        let _el: Ellipsoid = Ellipsoid::new(1.0, 0.0);
    }

    #[test]
    #[should_panic(expected = "Specified major semi-axis 'mjsa_m' should be greater than zero")]
    fn test_ellipsoid_from_descriptor_non_positive_major_semiaxis_panic() {
        let ed: EllipsoidDescriptor = EllipsoidDescriptor { mjsa_m: -1.0, ifltn: 0.0 };
        let _el: Ellipsoid = Ellipsoid::from_descriptor(&ed);
    }

    #[test]
    #[should_panic(expected = "Specifed inverse flattening 'ifltn' should be greater than zero")]
    fn test_ellipsoid_from_descriptor_inverse_flattening_out_of_range_panic() {
        let ed: EllipsoidDescriptor = EllipsoidDescriptor { mjsa_m: 1.0, ifltn: 0.0 };
        let _el: Ellipsoid = Ellipsoid::from_descriptor(&ed);
    }

    #[test]
    fn test_ellipsoid_new() {
        let el: Ellipsoid = Ellipsoid::new(WGS84_ELLIPSOID_DESCRIPTOR.mjsa_m, WGS84_ELLIPSOID_DESCRIPTOR.ifltn);        
        assert_approx_eq!(el.mjsa_m, WGS84_ELLIPSOID_DESCRIPTOR.mjsa_m, 1E-6);
        assert_approx_eq!(el.ifltn, WGS84_ELLIPSOID_DESCRIPTOR.ifltn, 1E-6);
        assert_approx_eq!(el.fltn, 1.0 / el.ifltn, 1E-6);
        assert_approx_eq!(el.mnsa_m, el.mjsa_m * (1.0 - el.fltn), 1E-6);
        assert_approx_eq!(el.ecnt, (el.mjsa_m.powi(2) - el.mnsa_m.powi(2)) / el.mjsa_m.powi(2), 1E-6);        
    }

    #[test]
    fn test_ellipsoid_from_descriptor() {
        let el: Ellipsoid = Ellipsoid::from_descriptor(&WGS84_ELLIPSOID_DESCRIPTOR);
        assert_approx_eq!(el.mjsa_m, WGS84_ELLIPSOID_DESCRIPTOR.mjsa_m, 1E-6);
        assert_approx_eq!(el.ifltn, WGS84_ELLIPSOID_DESCRIPTOR.ifltn, 1E-6);
        assert_approx_eq!(el.fltn, 1.0 / el.ifltn, 1E-6);
        assert_approx_eq!(el.mnsa_m, el.mjsa_m * (1.0 - el.fltn), 1E-6);
        assert_approx_eq!(el.ecnt, (el.mjsa_m.powi(2) - el.mnsa_m.powi(2)) / el.mjsa_m.powi(2), 1E-6);
    }

    #[test]
    fn test_wrap() {
        assert_eq!(wrap(10.0, 5.0), 5.0);
        assert_eq!(wrap(-10.0, 5.0), -5.0);
        assert_eq!(wrap(0.0, 5.0), 0.0);
        assert_eq!(wrap(5.0, 10.0), 5.0);
        assert_eq!(wrap(-5.0, 10.0), -5.0);
    }

    #[test]
    fn test_vincenty_equations() {

        let el: Ellipsoid = Ellipsoid::from_descriptor(&WGS84_ELLIPSOID_DESCRIPTOR);
        let mut sp_lat_rad: f64;
        let mut sp_lon_rad: f64;
        let mut ep_lat_rad: f64;
        let mut ep_lon_rad: f64;
        let mut fwd_az_rad: f64;
        let mut rev_az_rad: f64;
        let mut a_az_rad: f64;
        let mut a_raz_rad: f64;
        let mut d_point: (f64, f64, f64, i32);
        let mut dd_point: (f64, f64, f64, i32, bool);
        let mut adist_m: f64;        
    
        for lat_deg in -9..9 {
            sp_lat_rad = (lat_deg as f64 * 10.0).to_radians();
            for lon_deg in -18..18 {
                sp_lon_rad = (lon_deg as f64 * 10.0).to_radians();
                for az_deg in 0..35 {
                    fwd_az_rad = wrap_2pi((az_deg as f64 * 10.0).to_radians());
                    for dist_m in -1..3 {
                        let dst_m = (10.0 as f64).powi(dist_m);
                                                                        
                        d_point = vincenty_direct(sp_lat_rad, sp_lon_rad, fwd_az_rad, dst_m, &el, VNC_DEF_EPSILON, VNC_DEF_IT_LIMIT);
                                                
                        ep_lat_rad = d_point.0;
                        ep_lon_rad = d_point.1;
                        rev_az_rad = d_point.2;

                        dd_point = vincenty_inverse(sp_lat_rad, sp_lon_rad, ep_lat_rad, ep_lon_rad, &el, VNC_DEF_EPSILON, VNC_DEF_IT_LIMIT);
                        adist_m = dd_point.0;
                        a_az_rad = dd_point.1;
                        a_raz_rad = dd_point.2;

                        if ((a_az_rad - fwd_az_rad).abs() - PI2).abs() < 10E-3 {
                            assert_approx_eq!((a_az_rad - fwd_az_rad).abs(), PI2, 10E-6);                            
                        } else {
                            assert_approx_eq!(fwd_az_rad, a_az_rad, 10E-6);
                        }

                        assert_approx_eq!(rev_az_rad, a_raz_rad, 10E-6);                      
                        assert_approx_eq!(dst_m, adist_m, 10E-6);                        
                    }
                }
            }
        }        

    }
}