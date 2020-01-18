use crate::vector::Vector3;

#[derive(Debug, Default, Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray { origin, direction, time: 1.0 }
    }

    pub fn with_time(origin: Vector3, direction: Vector3, time: f32) -> Self {
        Ray { origin, direction, time }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}
