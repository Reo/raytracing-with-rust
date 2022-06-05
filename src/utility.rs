use rand::{thread_rng, Rng, ThreadRng};

/* Useful consts */
pub const EPS: f64 = 1024.0 * f64::EPSILON;
pub const INFTY: f64 = f64::INFINITY;
pub const PI: f64 = 3.141592653589793;

/* Utility functions */
#[inline(always)]
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

#[inline(always)]
pub fn unif_rng(a: f64, b: f64) -> f64 {
    let mut rng: ThreadRng;rng = thread_rng();
    return rng.gen_range(a,b);
}

#[inline(always)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
}

