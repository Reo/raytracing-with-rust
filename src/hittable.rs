use crate::ray::*;
use crate::vec::*;

pub struct HitList {
    pub p: Point3d, // 3d point where intersection occurs
    pub n: Vec3d,   // normal at point of intersection
    pub t: f64,     // ray parameter at point of intersection
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hitlist: &mut HitList) -> bool;
}

