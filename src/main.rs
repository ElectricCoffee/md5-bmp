extern crate md5;
extern crate bmp;
use md5::Digest;
use bmp::{Pixel, Image};
use bmp::consts::BLACK;
use std::env;

// Turns a digest into a 16-element array of pixels
fn make_pixels(digest: Digest) -> [Pixel; 16] {
    let mut arr: [Pixel; 16] = [BLACK; 16];
    for (i, &data) in digest.0.into_iter().enumerate() {
        arr[i] = Pixel::new(data, data, data);
    }

    arr
}

// Creates a square image containing the 16 colours in the hash.
fn make_image(side_length: u32, pixels: [Pixel; 16]) -> Image {
    let mut img = Image::new(side_length, side_length);
    let square_size: u32 = side_length / 4;
    for (x, y) in img.coordinates() {
        // clamp the x and y values to fit within the confines of the 4x4 color array
        let bounded_x: u32 = x / square_size;
        let bounded_y: u32 = y / square_size;
        // convert the 2D coordinate into a 1D index
        let color_index: u32 = 4 * bounded_y + bounded_x;
        let current_color = pixels[color_index as usize];
        img.set_pixel(x, y, current_color);
    }

    img
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("You did not supply enough arguments.");
        return;
    }
    
    if let Ok(canvas_size) = args[1].parse::<u32>() {
        let bytes = args[2].clone().into_bytes();
        let filename = args[3].clone();
        
        let checksum = md5::compute(bytes);
        let pixels = make_pixels(checksum);
        let img = make_image(canvas_size, pixels);
        
        if let Err(err) = img.save(filename) {
            println!("{}", err);
        }
    } else {
        println!("The input \"{}\" is not a valid integer.", args[1]);
    }
}
