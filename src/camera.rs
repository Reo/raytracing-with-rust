use crate::vec::{Point3d, Vec3d};
use crate::ray::Ray;

pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Point3d,
    horizontal: Vec3d,
    vertical: Vec3d,
    sw_corner: Point3d,
}

impl Camera {
    pub fn new_default() -> Self {
        let aspect_ratio = 16.0/9.0;
        let viewport_width = aspect_ratio * 2.0;
        let focal_length = 1.0;
        let h: Vec3d = Vec3d::new(viewport_width, 0.0, 0.0);
        let v: Vec3d = Vec3d::new(0.0, 2.0, 0.0);
        let corner: Vec3d = -(h/2.0 + v/2.0 + Vec3d::new(0.0, 0.0, focal_length));
        Camera {
            aspect_ratio: aspect_ratio,
            viewport_height: 2.0,
            viewport_width: viewport_width,
            focal_length: focal_length,
            origin: Point3d::zero(),
            horizontal: h,
            vertical: v,
            sw_corner: corner
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.sw_corner + u*self.horizontal + v*self.vertical - self.origin
        )
    }
}

