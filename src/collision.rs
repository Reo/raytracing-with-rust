use crate::vec::*;
use crate::ray::Ray;

pub fn hit_sphere(centre: Point3d, radius: f64, ray: Ray) -> f64 {
    let displacement = ray.origin() - centre;
    let a = dot(ray.dir(), ray.dir());
    let b = 2.0 * dot(displacement, ray.dir());
    let c = dot(displacement, displacement) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    // make sure discriminant is (practically) > 0
    if (discriminant - 2.0 * f64::EPSILON) > 0.0 {
        (-b - discriminant.sqrt() ) / (2.0 * a)
    } else {
        -1.0
    }
}
