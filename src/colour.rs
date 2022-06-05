use std::io::Write;
use std::io::BufWriter;
use std::fs::File;
use crate::vec::*;
use crate::utility::clamp;

pub fn write_colour(buf: &mut BufWriter<File>, pixel_colour: RGBcol, samples_per_pixel: i32) {
    // need to scale according to number of samples for basic anti-aliasing (division)
    // and scale for gamma correction (sqrt)
    let r: f64 = (pixel_colour.r()/(samples_per_pixel as f64)).sqrt();
    let g: f64 = (pixel_colour.g()/(samples_per_pixel as f64)).sqrt();
    let b: f64 = (pixel_colour.b()/(samples_per_pixel as f64)).sqrt();
    write!(*buf, "{} {} {}\n",
        (256.0 * clamp(r,0.0,0.99999)) as u8,
        (256.0 * clamp(g,0.0,0.99999)) as u8,
        (256.0 * clamp(b,0.0,0.99999)) as u8
    ).expect("Problem writing colour to ppm");
}

