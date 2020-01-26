use crate::camera::Camera;
use crate::hit::Hit;
use crate::material::Lambertian;
use crate::renderer::Renderer;
use crate::scene::{Scene, SceneSettings};
use crate::sphere::Sphere;
use crate::vector::Vector3;
use image::ImageFormat;
use std::rc::Rc;

mod aabb;
mod bvh;
mod camera;
mod hit;
mod material;
mod moving_sphere;
mod ray;
mod renderer;
mod scene;
mod sphere;
mod vector;

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

// Scene with many spheres
pub fn colored_sphere_scene() -> Vec<Box<dyn Hit>> {
    let mut hit: Vec<Box<dyn Hit>> = Vec::new();
    hit.push(Box::new(Sphere::new(
        Vector3::new(0.0, -1003.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.8))),
    )));

    for a in -5..=5 {
        for b in -5..=5 {
            for c in -5..=5 {
                let color = Vector3::new(
                    (a as f32 + 5.0) / 11.0,
                    (b as f32 + 5.0) / 11.0,
                    (c as f32 + 5.0) / 11.0,
                );

                let sp = Vector3::new(a as f32 * 0.5, b as f32 * 0.5, c as f32 * 0.5);
                hit.push(Box::new(Sphere::new(
                    sp,
                    0.2,
                    Rc::new(Lambertian::new(color)),
                )));
            }
        }
    }

    hit
}

fn main() {
    // Render
    let mut hit = colored_sphere_scene();
    let scene = Scene::new(
        &mut hit,
        SceneSettings {
            width: 500,
            height: 500,
            spp: 1,
            max_bounce: 50,
        },
    );

    let eye = Vector3::new(-5.5, 5.5, 5.5);
    let center = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::unit_y();
    let focus = (eye - center).length();
    let aperture = 0.1;

    let cam = Camera::new(
        eye,
        center,
        up,
        60.0,
        scene.settings.width as f32 / scene.settings.height as f32,
        aperture,
        focus,
        0.0,
        1.0,
    );

    let rend = Renderer::new(vec![scene], ImageFormat::Jpeg);
    rend.render(cam);
}
