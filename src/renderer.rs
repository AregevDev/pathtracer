use crate::camera::Camera;
use crate::hit::Hit;
use crate::random_float;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vector3;
use image::{ColorType, ImageFormat};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};

pub struct Renderer<'a> {
    scenes: Vec<Scene<'a>>,
    format: image::ImageFormat,
}

impl<'a> Renderer<'a> {
    // Move the scene vector here
    pub fn new(scenes: Vec<Scene<'a>>, format: ImageFormat) -> Self {
        Renderer { scenes, format }
    }

    pub fn render(&self, camera: Camera) {
        println!("Begin rendering process");

        self.scenes.iter().enumerate().for_each(|(idx, scene)| {
            println!("Rendering image {} out of {}", idx + 1, self.scenes.len());

            let out = self.render_to_pixels(idx, camera);
            let mut hasher = DefaultHasher::new();
            out.hash(&mut hasher);

            image::save_buffer_with_format(
                format!(
                    "temp/render_{:?}_{}.{}",
                    hasher.finish(),
                    idx,
                    format!("{:?}", self.format).to_lowercase()
                ),
                out.as_slice(),
                scene.settings.width as u32,
                scene.settings.height as u32,
                ColorType::Rgb8,
                self.format,
            )
            .map_err(|e| println!("Error writing to output image file {:?}", e.source()))
            .unwrap();
        });
    }

    fn color(&self, index: usize, ray: Ray, depth: i32) -> Vector3 {
        let scene = &self.scenes[index];
        let max_depth = scene.settings.max_bounce;

        if let Some(record) = scene.bvh.hit(ray, 0.0001, std::f32::MAX) {
            // Intersected
            if depth < max_depth {
                if let Some((scattered, attenuation)) = record.material.scatter(ray, &record) {
                    return attenuation * self.color(index, scattered, depth + 1);
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

    fn render_to_pixels(&self, index: usize, camera: Camera) -> Vec<u8> {
        let scene = &self.scenes[index];

        let width = scene.settings.width;
        let height = scene.settings.height;
        let spp = scene.settings.spp;

        let pixels = (0..height)
            .into_iter()
            .rev()
            .flat_map(|j| {
                (0..width).into_iter().flat_map(move |i| {
                    // Average color value
                    let mut col = Vector3::default();

                    // shoot ns rays for each sample and average the result
                    for _ in 0..spp {
                        // Normalized coordinates
                        let u = (i as f32 + random_float()) / width as f32;
                        let v = (j as f32 + random_float()) / height as f32;

                        let ray = camera.ray(u, v);

                        // Compute color
                        let c = self.color(index, ray, 0);
                        col += c;
                    }

                    // Divide by sample count
                    col /= spp as f32;

                    // Apply Gamma correction
                    col = Vector3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

                    // For each color component, convert to RGB values and add to the vector
                    (0..3)
                        .into_iter()
                        .map(move |c| (255.99 * col[c as usize]) as u8)
                })
            })
            .collect();

        pixels
    }
}
