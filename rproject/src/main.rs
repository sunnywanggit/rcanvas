use planetarium::{ Canvas,ImageFormat };
use std::{ fs, time::Instant };

fn main() {
    let start = Instant::now();
    let mut c = Canvas::new(100, 100);

    c.set_background(1000);
    c.clear();
    c.draw();

    // Export to a 8-bit gamma-compressed grayscale PNG image.
    let png_8bpp_bytes = c.export_image(ImageFormat::PngGamma8Bpp).unwrap();
    fs::write("./result.png", png_8bpp_bytes);
    println!("run time: {:?}", start.elapsed());
    // println!("{:?}", buffer);
}
