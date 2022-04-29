use crate::vec::*;
use crate::hittable::HitList;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct Sphere {
    pub centre: Point3d,
    pub radius: f64,
}

impl Sphere {
    pub fn new(c: Point3d, r: f64) -> Sphere { Sphere { centre: c, radius: r } }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hitlist: &mut HitList) -> bool {
        let displacement = ray.origin() - self.centre;
        let a = dot(ray.dir(), ray.dir());
        let half_b = dot(displacement, ray.dir());
        let c = dot(displacement, displacement) - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;

        // make sure discriminant is non-negative and nontrivially greater than 0
        if (discriminant - 16.0 * f64::EPSILON) < 0.0 {
            return false
        }
        let sqrt_disc = discriminant.sqrt();

        // find nearest root in the acceptable range
        // mutable just to try both candidates
        let mut root = -((half_b + sqrt_disc)/a);
        if root < t_min || root > t_max {
            // in this case, the first root doesn't qualify. Try again
            root = (-half_b + sqrt_disc)/a;
            if root < t_min || root > t_max {
                // in this case, neither root qualifies
                return false;
            }
        }

        hitlist.t = root;                         // add intersection ray param to the hitlist
        hitlist.p = ray.at(root);                     // add intersection point to the hitlist
        hitlist.n = (hitlist.p - self.centre) / self.radius; // compute normal at intersection

        return true;
    }
}

