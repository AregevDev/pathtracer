//TODO: Add more comments

use std::fmt::Write;

use crate::camera::Camera;
use crate::hit::Hit;
use crate::hit_list::HitList;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vector3;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;

mod camera;
mod hit;
mod hit_list;
mod material;
mod ray;
mod sphere;
mod vector;

fn random_double() -> f64 {
    let mut rnd = rand::thread_rng();
    rnd.gen::<f64>()
}

// Generate a random point in the unit sphere
fn random_in_unit_sphere() -> Vector3 {
    let mut p;

    loop {
        p =
            2.0 * Vector3::new(
                random_double() as f32,
                random_double() as f32,
                random_double() as f32,
            ) - Vector3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break;
        }
    }

    p
}

// Compute the ray's color
fn color(ray: Ray, world: &HitList, depth: i32) -> Vector3 {
    if let Some(record) = world.hit(ray, 0.001, std::f32::MAX) {
        if let Some(scattered) = record.material.scatter(ray, &record) {
            if depth < 50 {
                return scattered.color * color(scattered.ray.unwrap(), world, depth + 1);
            }
        }
    }

    let nd = ray.direction.normalize();
    let t = 0.5 * (nd.y + 1.0);
    Vector3::new(1.0, 1.0, 1.0).lerp(Vector3::new(0.5, 0.7, 1.0), t)
}

fn main() {
    // Define image dimensions
    let image_width = 512;
    let image_height = 512;
    let samples = 100;

    // Total rays to shoot
    let total = image_width * image_height * samples;

    // Output filename
    let filename = "test.ppm";

    // Allocate a string to hold the image data
    let mut out = String::with_capacity(image_width * image_height);

    // Create progress bar
    let prog = ProgressBar::new(total as u64);
    prog.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {percent}%/100% ({eta})")
            .progress_chars("#>-"),
    );

    // Write the PPM file headers
    writeln!(out, "P3\n{} {}\n255", image_width, image_height).unwrap();

    // Define camera
    let camera = Camera::new();

    // Define world of spheres
    let mut hl = HitList::new();
    hl.add(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        Material::lambertian(Vector3::new(0.1, 0.8, 0.2)),
    ));
    hl.add(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        Material::lambertian(Vector3::new(0.1, 0.1, 0.1)),
    ));
    hl.add(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        Material::metal(Vector3::new(0.0, 1.0, 1.0), 0.0),
    ));
    hl.add(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::metal(Vector3::new(1.0, 0.0, 1.0), 0.2),
    ));
    hl.add(Sphere::new(
        Vector3::new(-1.5, 0.5, -2.5),
        1.5,
        Material::lambertian(Vector3::new(1.0, 0.467, 0.0)),
    ));
    hl.add(Sphere::new(
        Vector3::new(1.5, 0.5, -2.5),
        1.5,
        Material::lambertian(Vector3::new(0.702, 0.78, 0.471)),
    ));

    // Write pixel data
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut col = Vector3::new(0.0, 0.0, 0.0);

            // Average the pixel colors from each sample
            for _ in 0..samples {
                // Normalize colors from 0.0 to 1.0
                let u = (i as f32 + random_double() as f32) / image_width as f32;
                let v = (j as f32 + random_double() as f32) / image_height as f32;

                let ray = camera.get_ray(u, v);
                col += color(ray, &hl, 0);
            }

            // Average
            col /= samples as f32;
            col = Vector3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            // Convert from normalized to RGB
            let ir = (255.0 * col.x) as i32;
            let ig = (255.0 * col.y) as i32;
            let ib = (255.0 * col.z) as i32;

            // Write RGB set into the file
            writeln!(out, "{} {} {}", ir, ig, ib).unwrap();

            // Update progress bar
            prog.inc(1);
        }
    }

    // Finish the progress bar
    prog.finish_with_message(format!("Rendered to {}", filename).as_str());

    // Finally, write the entire chunk of pixel data to the file at one time
    std::fs::write("test.ppm", out).expect("Failed to write output image");
}
