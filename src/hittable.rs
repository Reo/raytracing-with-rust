use crate::ray::*;
use crate::vec::*;
use crate::material::*;

// node containing intersection information
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitNode {
    pub p: Point3d,              // 3d point where intersection occurs
    pub n: Vec3d,                // normal at point of intersection
    pub t: f64,                  // ray parameter at point of intersection
    pub is_front: bool,          // for determining whether ray is inside object
    pub material: *mut Material, // material used for determining object's appearance
    pub next_dir: Ray,           // next direction ray is headed
}

impl HitNode {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3d) {
       self.is_front = dot(r.dir(), outward_normal) < 0.0;
       self.n = if self.is_front { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hitnode: &mut HitNode) -> bool;
}

