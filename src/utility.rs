
/* Useful consts */
const EPS: f64 = 32.0 * f64::EPSILON;
const INFTY: f64 = f64::INFINITY;
const PI: f64 = 3.141592653589793;

/* Utility functions */
#[inline(always)]
fn deg_to_rad(deg: f64) {
    deg * PI / 180.0
}

