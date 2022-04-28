use std::io;
use std::io::Write;
use std::io::BufWriter;
use std::fs::File;

mod vec;
mod colour;
mod ray;
mod collision;

fn ray_colour(r: ray::Ray) -> vec::RGBcol {
    // check where ray intersects with sphere.
    // Consider only positive values ie. those in front of the camera
    let t = collision::hit_sphere(vec::Point3d::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = vec::normalise(r.at(t) - vec::Vec3d::new(0.0, 0.0, -1.0));
        return 0.5*vec::RGBcol::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }
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
