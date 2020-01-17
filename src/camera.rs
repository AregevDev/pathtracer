use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug, Default, Copy, Clone)]
pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3,
}

impl Camera {
    pub fn new(
        lower_left_corner: Vector3,
        horizontal: Vector3,
        vertical: Vector3,
        origin: Vector3,
    ) -> Self {
        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v,
        )
    }
}
