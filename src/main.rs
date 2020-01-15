use crate::vector::Vector3;
use std::fmt::Write;
use std::fs;

mod vector;

// Compute the final color
fn color() -> Vector3 {
    Vector3::default()
}

fn main() {
    // Output properties
    let filename = "test.ppm";
    let nx = 500;
    let ny = 500;

    // Output buffer
    let mut out = String::with_capacity(nx * ny);

    // Write PPM headers
    writeln!(out, "P3\n{} {}\n255", nx, ny).unwrap();

    // Main loop
    // Render left to right, top to bottom
    for j in (0..ny).rev() {
        for i in 0..nx {
            // Normalize
            let col = Vector3::new(
                i as f32 / nx as f32,
                j as f32 / ny as f32,
                1.0,
            );

            // Convert to RGB
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            // Write output pixels
            writeln!(out, "{} {} {}", ir, ig, ib).unwrap();
        }
    }

    // Write to ppm file
    fs::write(filename, out).unwrap();
}
