use crate::random_in_unit_disk;
use crate::ray::Ray;
use crate::vector::Vector3;
use std::f32::consts::PI;

#[derive(Debug, Default, Copy, Clone)]
pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    w: Vector3,
    u: Vector3,
    v: Vector3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        eye: Vector3,
        center: Vector3,
        up: Vector3,
        fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (eye - center).normalize();
        let u = (up.cross(w)).normalize();
        let v = w.cross(u);

        Camera {
            origin: eye,
            lower_left_corner: eye
                - u * focus_dist * half_width
                - v * focus_dist * half_height
                - w * focus_dist,
            horizontal: u * half_width * focus_dist * 2.0,
            vertical: v * half_height * focus_dist * 2.0,
            w,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
