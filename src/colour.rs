use std::io::Write;
use std::io::BufWriter;
use std::fs::File;
use crate::vec::*;

pub fn write_colour(buf: &mut BufWriter<File>, pixel_colour: RGBcol) {
    write!(*buf, "{} {} {}\n",
        (255.000 * pixel_colour.r()) as i32,
        (255.000 * pixel_colour.g()) as i32,
        (255.000 * pixel_colour.b()) as i32
    ).expect("Problem writing colour to ppm");
}

