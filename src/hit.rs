use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug, Default, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
}

pub trait Hit {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}
