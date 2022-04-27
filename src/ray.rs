use crate::vec::*;

#[derive(Debug, Copy, CLone, PartialEq)]
pub struct Ray {
    const orig: Point3d,
    const dir: Vec3d
}

impl Ray {
    pub origin(self) -> Point3d { self.orig };
    pub dir(self) -> Vec3d { self.dir };
    pub at(self, t: f64) -> Point3d { self.orig + t*self.dir }
}

