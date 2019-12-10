use std::fmt::Write;
use std::fs;

use crate::camera::Camera;
use crate::hit::Hit;
use crate::hit_list::HitList;
use crate::material::{Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vector3;
use rand::Rng;
use std::time::Instant;

mod camera;
mod hit;
mod hit_list;
mod material;
mod ray;
mod sphere;
mod vector;

fn random_in_unit_sphere() -> Vector3 {
    let mut rnd = rand::thread_rng();
    let mut p;

    loop {
        p = Vector3::new(rnd.gen::<f32>(), rnd.gen::<f32>(), rnd.gen::<f32>()) * 2.0
            - Vector3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break;
        }
    }

    p
}

fn compute_color(r: Ray, world: &HitList, depth: i32) -> Vector3 {
    let (hit, record) = world.hit(r, 0.001, std::f32::MAX);

    if hit {
        let (scattered, attenuation, ray_out) = record.material.borrow().scatter(r, &record);
        if depth < 50 && scattered {
            return attenuation * compute_color(ray_out, &world, depth + 1) * 0.5;
        }
    }

    let unit_dir = r.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Create a string to hold the PPM data
    let mut img_data = String::with_capacity(100000);

    // Random generator
    let mut rnd = rand::thread_rng();

    // Set image width and height
    let width = 500;
    let height = 500;
    let samples = 1;

    // Define camera
    let cam = Camera::new();

    // Define world
    let mut hl = HitList::new();
    hl.add(Sphere::new(
        Vector3::new(0.0, 0.0, -1.5),
        0.5,
        Lambertian::new(Vector3::new(0.8, 0.3, 0.3)),
    ));
    hl.add(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.5),
        0.5,
        Metal::new(Vector3::new(1.0, 1.0, 1.0), 0.0),
    ));
    hl.add(Sphere::new(
        Vector3::new(2.5, 1.0, -2.5),
        0.5,
        Metal::new(Vector3::new(0.0, 0.8, 0.8), 0.0),
    ));
    hl.add(Sphere::new(
        Vector3::new(0.0, -100.5, -2.0),
        100.0,
        Lambertian::new(Vector3::new(0.8, 0.8, 0.0)),
    ));
    hl.add(Sphere::new(
        Vector3::new(0.45, 0.0, -3.0),
        1.4,
        Metal::new(Vector3::new(0.027, 0.863, 0.145), 0.0),
    ));

    // Start counting
    let inst = Instant::now();

    // Write the file headers
    write!(&mut img_data, "P3\n{} {}\n255\n", width, height).unwrap();

    // Write pixels
    for j in (0..=height).rev() {
        for i in 0..width {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                // Normalized UV coordinates
                let u = (i as f32 + rnd.gen::<f32>()) / width as f32;
                let v = (j as f32 + rnd.gen::<f32>()) / height as f32;

                let r = cam.get_ray(u, v);
                col += compute_color(r, &hl, 0);
            }

            col /= samples as f32;
            col = col.sqrt();

            let ir = (255.0 * col.x) as i32;
            let ig = (255.0 * col.y) as i32;
            let ib = (255.0 * col.z) as i32;

            write!(&mut img_data, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    // Write the entire image data to the file
    fs::write("test.ppm", img_data).unwrap();

    // Measure how long it rendered
    let duration = inst.elapsed().as_secs_f32();
    println!(
        "Image written successfully!\nRendered in {:.1} minutes",
        duration / 60.0
    );
}
