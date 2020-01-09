use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug)]
pub struct Camera {
    pub origin: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub corner: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            corner: Vector3::new(-2.0, -1.0, -1.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 4.0, 0.0),
            origin: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
