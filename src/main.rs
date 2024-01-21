use image::{self, ImageFormat::Avif};
use std::io::Cursor;

fn main() {
    let jpg_file = std::path::Path::new("test/c001_1.jpg");

    println!("Opening \"{}\"...", jpg_file.display());

    // open file
    let jpg_img: image::DynamicImage = image::io::Reader::open(&jpg_file)
        .expect("File not found")
        .decode()
        .expect("Error decoding image");

    // save image as avif format
    let av1_img = jpg_img
        .save_with_format("test/c001_1.avif", Avif)
        .expect("Error writing avif file");
}
