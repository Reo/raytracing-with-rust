use crate::vec::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    orig: Point3d,
    dir: Vec3d,
}

impl Ray {
    pub fn new(o: Point3d, d: Vec3d) -> Self { Ray { orig:o, dir:d } }
    pub fn origin(self) -> Point3d { self.orig }
    pub fn dir(self) -> Vec3d { self.dir }
    pub fn at(self, t: f64) -> Point3d { self.orig + t*self.dir }
}

