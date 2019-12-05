use std::fmt::Write;
use std::fs;

use ray::Ray;
use vector::Vector3;

mod ray;
mod vector;

fn hit_sphere(center: Vector3, radius: f32, r: Ray) -> bool {
    let oc = r.origin - center;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn compute_color(r: Ray) -> Vector3 {
    // Draw a pastel red sphere
    if hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vector3::new(1.0, 0.0, 0.0) * 0.5 + 0.5;
    }

    // Draw pastel blue sphere
    if hit_sphere(Vector3::new(-0.5, 0.0, -1.0), 0.5, r) {
        return Vector3::new(0.0, 0.0, 1.0) * 0.5 + 0.5;
    }

    // Draw pastel green sphere
    if hit_sphere(Vector3::new(0.5, 0.0, -1.0), 0.5, r) {
        return Vector3::new(0.0, 1.0, 0.0) * 0.5 + 0.5;
    }

    let unit_dir = r.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.5, 1.0) * t
}

fn main() {
    // Create a string to hold the PPM data
    let mut img_data = String::new();

    // Set image width and height
    let width = 500;
    let height = 500;

    // Define camera vectors
    let ll_corner = Vector3::new(-1.0, -1.0, -1.0);
    let horizontal = Vector3::new(2.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    // Write the file headers
    write!(&mut img_data, "P3\n{} {}\n255\n", width, height).unwrap();

    // Write pixels
    for j in (0..=height).rev() {
        for i in 0..width {
            // Normalized UV coordinates
            let u = i as f32 / width as f32;
            let v = j as f32 / height as f32;

            let r = Ray::new(origin, ll_corner + horizontal * u + vertical * v);
            let col = compute_color(r);

            let ir = (255.0 * col.x) as i32;
            let ig = (255.0 * col.y) as i32;
            let ib = (255.0 * col.z) as i32;

            write!(&mut img_data, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    // Write the entire image data to the file
    fs::write("test.ppm", img_data).unwrap();
}
