use std::fs;
use std::fmt::Write;

fn main() {
    // Create a string to hold the PPM data
    let mut img_data = String::new();

    // Set image width and height
    let width = 100;
    let height = 100;

    // Write the file headers
    write!(&mut img_data, "P3\n{} {}\n255\n", width, height).unwrap();

    // Write pixels
    for j in 0..width {
        for i in 0..height {
            let red = (i as f32) / (width as f32);
            let green = (j as f32) / (height as f32);
            let blue = 1.0f32;

            let ir = (255.0 * red) as i32;
            let ig = (255.0 * green) as i32;
            let ib = (255.0 * blue) as i32;

            write!(&mut img_data, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    // Write the entire image data to the file
    fs::write("test.ppm", img_data).unwrap();
}
