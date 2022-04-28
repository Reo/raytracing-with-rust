use crate::vec::*;
use crate::ray::Ray;

pub fn hit_sphere(centre: Point3d, radius: f64, ray: Ray) -> f64 {
    let displacement = ray.origin() - centre;
    let a = dot(ray.dir(), ray.dir());
    let half_b = dot(displacement, ray.dir());
    let c = dot(displacement, displacement) - radius*radius;
    let discriminant = half_b*half_b - a*c;
    // make sure discriminant is (practically) > 0
    if (discriminant - 2.0 * f64::EPSILON) > 0.0 {
        (-half_b - discriminant.sqrt() ) / a
    } else {
        -1.0
    }
}
