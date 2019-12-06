use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord::default()
    }
}

pub trait Hit {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> (bool, HitRecord);
}
