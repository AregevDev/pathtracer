use crate::ray::Ray;
use crate::vector::Vector3;
use std::fmt::Write;
use std::fs;
use crate::record::{HitRecord, World};
use crate::hit::{Hit, Sphere};

mod hit;
mod ray;
mod record;
mod vector;

// Ray-Sphere intersection, return hit point, -1 if not
fn hit_sphere(center: Vector3, radius: f32, ray_in: Ray) -> f32 {
    let oc = ray_in.origin - center;
    let a = ray_in.direction.dot(ray_in.direction);
    let b = oc.dot(ray_in.direction) * 2.0;
    let c = oc.dot(oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }

    (-b - discriminant.sqrt()) / (2.0 * a)
}

// Compute the final color
fn color(ray: Ray, world: &World) -> Vector3 {
    let mut record = HitRecord::default();
    if world.hit(ray, 0.0, std::f32::MAX, &mut record) { // Check for intersection
        return Vector3::new(record.normal.x + 1.0, record.normal.y + 1.0, record.normal.z + 1.0) * 0.5; // Shade with normals from the hit record
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

    // Camera vectors
    let lower_left_corner = Vector3::new(-2.0, -2.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 4.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

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
            // Normalized coordinates
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            // Shoot a ray to the pixel
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            // Compute color
            let col = color(ray, &world);

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
