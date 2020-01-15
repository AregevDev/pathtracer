use crate::ray::Ray;
use crate::vector::Vector3;
use std::fmt::Write;
use std::fs;

mod ray;
mod vector;

// Compute the final color
fn color(ray: Ray) -> Vector3 {
    let dir = ray.direction.normalize();
    let t = 0.5 * (dir.y + 1.0);

    Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Output properties
    let filename = "test.ppm";
    let nx = 500;
    let ny = 500;

    // Camera vectors
    let lower_left_corner = Vector3::new(-2.0, -2.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 4.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    // Output buffer
    let mut out = String::with_capacity(nx * ny);

    // Write PPM headers
    writeln!(out, "P3\n{} {}\n255", nx, ny).unwrap();

    // Main loop
    // Render left to right, top to bottom
    for j in (0..ny).rev() {
        for i in 0..nx {
            // Normalized coordinates
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            // Shoot a ray to the pixel
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            // Compute color
            let col = color(ray);

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
