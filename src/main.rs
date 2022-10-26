use std::path::PathBuf;

use clap::Parser;
use image::GenericImageView;

#[derive(Parser, Debug)]
struct Args {
    /// The image to convert
    input_image: PathBuf,
}

fn main() {
    let args = Args::parse();
    let input_image_path = args.input_image;

    let in_image = match image::open(&input_image_path) {
        Ok(x) => x,
        Err(why) => {
            eprintln!("Could not open {input_image_path:?} - {why}");
            return;
        }
    };

    let image_width = in_image.dimensions().0;
    let image_height = in_image.dimensions().1;

    let mut raw_pixels = vec![0u16; (image_width * image_height) as usize];

    for (x, y, color) in in_image.pixels() {
        let pixel_index = (y * image_width + x) as usize;

        // 5 bits per pixel
        let r5: u16 = ((color.0[0] & 0b11111000) >> 3).into();
        let g5: u16 = ((color.0[1] & 0b11111000) >> 3).into();
        let b5: u16 = ((color.0[2] & 0b11111000) >> 3).into();

        let color_short: u16 = (r5) | (g5 << 5) | (b5 << 10);
        raw_pixels[pixel_index] = color_short;
    }

    // print result as a C-like array, e.g. "{ 0x1234, 0xFFFF }"
    let c_array = raw_pixels
        .iter()
        .map(|x| format!("0x{x:04X}"))
        .collect::<Vec<String>>();
    println!("{{ {} }}", c_array.join(", "));
}
