use std::io;
use std::io::Write;
use std::io::BufWriter;
use std::fs::File;

use crate::hittable::Hittable;

mod vec;
mod colour;
mod ray;
mod sphere;
mod hittable;

fn ray_colour(r: ray::Ray) -> vec::RGBcol {
    // check where ray intersects with sphere.
    // Consider only positive values ie. those in front of the camera

    // here's a sphere intersection for example:
    let sphere_centre = vec::Point3d::new(0.0, 0.0, -1.0);
    let my_sphere = sphere::Sphere::new(sphere_centre, 0.5);
    // intersections
    let mut hitlist = hittable::HitNode { p: vec::Point3d::zero(), n: vec::Vec3d::zero(), t: 0.0, is_front: true };
    let t_min = 0.000001;
    let t_max = 1000000.0;
    let hit_an_object = my_sphere.hit(&r, t_min, t_max, &mut hitlist);

    if hit_an_object {
        return 0.5*vec::RGBcol::new(hitlist.n.x() + 1.0, hitlist.n.y() + 1.0, hitlist.n.z() + 1.0);
    }

    // otherwise, return a background colour/gradient
    let unit_dir = vec::normalise(r.dir());
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t)*vec::RGBcol::new(1.0,1.0,1.0) + t*vec::RGBcol::new(0.5,0.7,1.0)
}

fn main() {
    // image details
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as i32;
    let filename = "out.ppm";

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec::Point3d::zero();
    let horizontal = vec::Vec3d::new(viewport_width, 0.0, 0.0);
    let vertical = vec::Vec3d::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vec::Vec3d::new(0.0, 0.0, focal_length);
    // image handle
    let f = File::create(filename).expect("Unable to create file");
    let mut buf = BufWriter::new(f);

    /* render */
    // ppm header
    write!(buf, "P3\n{} {}\n255\n", img_width, img_height).expect("Problem writing header to ppm");

    for j in (0..img_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();
        for i in 0..img_width {
            let u = (i as f64) / (img_width - 1) as f64;
            let v = (j as f64) / (img_height - 1) as f64;
            let r = ray::Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_col = ray_colour(r);
            colour::write_colour(&mut buf, pixel_col);
        }
    }
    eprintln!("\nDone!\n");
}
