use std::io;
use std::io::Write;
use std::io::BufWriter;
use std::fs::File;

mod vec;
mod colour;

fn main() {
    // image details
    let img_width = 256;
    let img_height = 256;
    let filename = "out.ppm";

    // image handle
    let f = File::create(filename).expect("Unable to create file");
    let mut buf = BufWriter::new(f);

    // ppm header
    write!(buf, "P3\n{} {}\n255\n", img_width, img_height).expect("Problem writing header to ppm");
    // render
    for j in (0..img_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();
        for i in 0..img_width {
            let pixel_col = vec::RGBcol::new(
                (i as f64) / (img_width - 1) as f64,
                (j as f64) / (img_height - 1) as f64,
                0.25);
            colour::write_colour(&mut buf, pixel_col);
        }
    }
    eprintln!("\nDone!\n");
}
