use std::io;
use std::io::Write;
use std::io::BufWriter;
use std::fs::File;

use crate::hittable::Hittable;

mod camera;
mod colour;
mod hitlist;
mod hittable;
mod ray;
mod sphere;
mod utility;
mod vec;

fn ray_colour(r: ray::Ray, world: &Vec<&dyn Hittable>, depth: i32) -> vec::RGBcol {
    // break on max recursion depth
    if depth == 0 { return vec::RGBcol::zero() }
    // check where ray intersects with sphere.
    // Consider only positive values ie. those in front of the camera

    // intersections
    // initialise the hitnode to a zero value
    let mut hitnode = hittable::HitNode { p: vec::Point3d::zero(), n: vec::Vec3d::zero(), t: 0.0, is_front: true };
    let hit_an_object = world.hit(&r, utility::EPS, utility::INFTY, &mut hitnode);

    if hit_an_object {
        // TODO currently only doing diffuse objects so rays always bounce in random direction
        // also, currently doing point + normal which assumes we're outside the object
        // would be point - normal if we were inside
        let next_dir: vec::Vec3d = hitnode.p + hitnode.n + vec::Vec3d::rand_unit();
        return 0.5 * ray_colour(ray::Ray::new(hitnode.p, next_dir - hitnode.p), world, depth - 1);
        // DEBUG
        // colour normals: hitnode.n + vec::RGBcol::new(1.0,1.0,1.0)
    }

    // otherwise, return a background colour/gradient
    let t = 0.5 * (vec::normalise(r.dir()).y() + 1.0);
    return (1.0 - t)*vec::RGBcol::new(1.0,1.0,1.0) + t*vec::RGBcol::new(0.5,0.7,1.0);
}

fn main() {
    // image details
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as i32;
    let filename = "out.ppm";
    let samples_per_pixel = 100;
    let max_depth: i32 = 5;

    // define the world
    let mut world : Vec<&dyn Hittable> = vec![];
    // **** objects in the world
    let main_sphere : sphere::Sphere = sphere::Sphere {
        centre: vec::Point3d::new(0.0, 0.0, -1.0),
        radius: 0.5
    };
    let floor_sphere : sphere::Sphere = sphere::Sphere {
        centre: vec::Point3d::new(0.0, -100.5, -1.0),
        radius: 100.0
    };
    world.push(&main_sphere);
    world.push(&floor_sphere);

    // define the camera(s)
    let camera: camera::Camera = camera::Camera::new_default();

    // image handle
    let f = File::create(filename).expect("Unable to create file");
    let mut buf = BufWriter::new(f);

    /* render */
    // ppm header
    write!(buf, "P3\n{} {}\n255\n", img_width, img_height).expect("Problem writing header to ppm");

    // rows
    for j in (0..img_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();
        // columns
        for i in 0..img_width {
            let mut pixel_col: vec::RGBcol = vec::RGBcol::zero();
            for _k in 0..samples_per_pixel {
                let u = (i as f64 + utility::unif_rng(0.0,1.0)) / (img_width - 1) as f64;
                let v = (j as f64 + utility::unif_rng(0.0,1.0)) / (img_height - 1) as f64;
                // get corresponding ray from camera to viewport
                let r: ray::Ray = camera.get_ray(u, v);
                pixel_col += ray_colour(r, &world, max_depth);
            }
            colour::write_colour(&mut buf, pixel_col, samples_per_pixel);
        }
    }
    eprintln!("\nDone!\n");
}

