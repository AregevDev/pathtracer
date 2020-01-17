use crate::ray::Ray;
use crate::vector::Vector3;
use std::f32::consts::PI;

#[derive(Debug, Default, Copy, Clone)]
pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(eye: Vector3, center: Vector3, up: Vector3, fov: f32, aspect: f32) -> Self {
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (eye - center).normalize();
        let u = (up.cross(w)).normalize();
        let v = w.cross(u);

        Camera {
            origin: eye,
            lower_left_corner: eye - u * half_width - v * half_height - w,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
        }
    }

    pub fn ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin,
        )
    }
}
