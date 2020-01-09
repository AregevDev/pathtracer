use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector3;
use std::fmt::Debug;

#[derive(Debug, Default, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

pub trait Hit {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
