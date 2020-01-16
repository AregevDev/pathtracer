use crate::camera::Camera;
use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vector3;
use crate::world::World;
use std::fmt::Write;
use std::fs;

mod camera;
mod hit;
mod ray;
mod sphere;
mod vector;
mod world;

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

        if p.squared_length() >= 1.0 {
            return p;
        }
    }
}

// Compute the final color
fn color(ray: Ray, world: &World) -> Vector3 {
    let mut record = HitRecord::default();
    if world.hit(ray, 0.001, std::f32::MAX, &mut record) {
        // Check for intersection
        let target = record.p + record.normal + random_in_unit_sphere();
        return color(Ray::new(record.p, target - record.p), &world) * 0.5;
    }

    let dir = ray.direction.normalize(); // Normalize ray direction
    let t = 0.5 * (dir.y + 1.0); // Place t between -1 and 1

    Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t // Interpolate
}

fn main() {
    // Output properties
    let filename = "test.ppm";
    let nx = 500;
    let ny = 500;
    let ns = 10;

    // Camera vectors
    let lower_left_corner = Vector3::new(-2.0, -2.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 4.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(lower_left_corner, horizontal, vertical, origin);

    // World
    let mut world = World::new();
    world.add(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vector3::new(1.0, -100.5, -1.0), 100.0));

    // Output buffer
    let mut out = String::with_capacity(nx * ny);

    // Write PPM headers
    writeln!(out, "P3\n{} {}\n255", nx, ny).unwrap();

    // Main loop
    // Render left to right, top to bottom
    for j in (0..ny).rev() {
        for i in 0..nx {
            // Average color value
            let mut col = Vector3::default();

            // shoot ns rays for each sample and average the result
            for s in 0..ns {
                // Normalized coordinates
                let u = (i as f32 + random_float()) / nx as f32;
                let v = (j as f32 + random_float()) / ny as f32;

                let ray = camera.ray(u, v);

                // Compute color
                let c = color(ray, &world);
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

    // Write to ppm file
    fs::write(filename, out).unwrap();
}
