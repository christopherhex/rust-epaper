mod rpi_epaper; 

extern crate image;

use crate::rpi_epaper::epaper_test;
use image::{GenericImageView, ImageBuffer, RgbImage};

fn main() {
    println!("Hello, world!");
    let img = image::open("data/epaper_test.png").unwrap();

    // Convert to grayscale
    let luma = img.into_luma();
    let raw = luma.into_raw();

    println!("Run Test Script");
    epaper_test::inittest(&raw);
}
