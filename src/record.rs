use crate::hit::Hit;
use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug, Default, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
}
