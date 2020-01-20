use crate::hit::Hit;
use crate::ray::Ray;
use crate::scenes::{basic_scene, random_scene, colored_sphere_scene};
use crate::vector::Vector3;
use crate::world::World;
use std::fmt::Write;
use std::fs;
use chrono::Utc;
use std::time::{Instant, SystemTime};

mod aabb;
mod camera;
mod hit;
mod material;
mod moving_sphere;
mod ray;
mod scenes;
mod sphere;
mod vector;
mod world;
mod bvh;

// Generate a random float
pub fn random_float() -> f32 {
    rand::random::<f32>()
}

// Generate a random point in 3D space, discard if outside of the unit sphere
pub fn random_in_unit_sphere() -> Vector3 {
    let mut p;

    loop {
        p = Vector3::new(random_float(), random_float(), random_float()) * 2.0
            - Vector3::new(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

// Generate a random point in 2D space, discard if outside of the unit circle
pub fn random_in_unit_disk() -> Vector3 {
    let mut p;

    loop {
        p = Vector3::new(random_float(), random_float(), 0.0) * 2.0 - Vector3::new(1.0, 1.0, 0.0);

        if p.dot(p) < 1.0 {
            return p;
        }
    }
}

// Compute the final color
fn color(ray: Ray, world: &Box<dyn Hit>, depth: i32) -> Vector3 {
    if let Some(record) = world.hit(ray, 0.0001, std::f32::MAX) {
        // Intersected
        if depth < 50 {
            if let Some((scattered, attenuation)) = record.material.scatter(ray, &record) {
                return attenuation * color(scattered, world, depth + 1);
            }
            return Vector3::default();
        }
    }

    // Background gradient
    let dir = ray.direction.normalize(); // Normalize ray direction
    let t = 0.5 * (dir.y + 1.0); // Place t between -1 and 1

    // Interpolate
    return Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Output properties
    let filename = "test.ppm";
    let nx = 500;
    let ny = 500;
    let ns = 100;

    // Scene
    let (world, cam) = colored_sphere_scene(nx, ny);

    // Output buffer
    let mut out = String::with_capacity(nx * ny);

    let last = SystemTime::now();

    // Write PPM headers
    writeln!(out, "P3\n{} {}\n255", nx, ny).unwrap();

    // Main loop
    // Render left to right, top to bottom
    for j in (0..ny).rev() {
        for i in 0..nx {
            // Average color value
            let mut col = Vector3::default();

            // shoot ns rays for each sample and average the result
            for _ in 0..ns {
                // Normalized coordinates
                let u = (i as f32 + random_float()) / nx as f32;
                let v = (j as f32 + random_float()) / ny as f32;

                let ray = cam.ray(u, v);

                // Compute color
                let c = color(ray, &world, 0);
                col += c;
            }

            // Divide by sample count
            col /= ns as f32;

            // Apply Gamma correction
            col = Vector3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            // Convert to RGB
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            // Write output pixels
            writeln!(out, "{} {} {}", ir, ig, ib).unwrap();
        }
    }

    println!("{:?}", last.elapsed().unwrap());

    // Write to PPM file
    fs::write(filename, out).unwrap();
}
