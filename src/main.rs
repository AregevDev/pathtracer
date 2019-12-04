mod vector;

use vector::Vector3;

use std::fmt::Write;
use std::fs;

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
            let col = Vector3::new(
                (i as f32) / (width as f32),
                (j as f32) / (height as f32),
                1.0,
            );

            let ir = (255.0 * col.x) as i32;
            let ig = (255.0 * col.y) as i32;
            let ib = (255.0 * col.z) as i32;

            write!(&mut img_data, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    // Write the entire image data to the file
    fs::write("test.ppm", img_data).unwrap();
}
