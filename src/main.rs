extern crate md5;
extern crate bmp;
use bmp::consts::BLACK;

// Turns a digest into a 16-element array of pixels
fn make_pixels(digest: md5::Digest) -> [bmp::Pixel; 16] {
    let mut arr: [bmp::Pixel; 16] = [BLACK; 16];
    for (i, &data) in digest.0.into_iter().enumerate() {
        arr[i] = bmp::Pixel{r: data, g: data, b: data};
    }

    arr
}

// Creates a square image containing the 16 colours in the hash.
fn make_image(side_length: u32, pixels: [bmp::Pixel; 16]) -> bmp::Image {
    let mut img = bmp::Image::new(side_length, side_length);
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
    let vec = md5::compute(b"Nikolaj Lepka");
    let gray = make_pixels(vec);
    let img = make_image(256, gray);
    if let Err(err) = img.save("img.bmp") {
        println!("{}", err);
    }
}
