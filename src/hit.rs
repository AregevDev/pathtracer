use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector3;
use std::rc::Rc;

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(t: f32, p: Vector3, normal: Vector3, material: Rc<dyn Material>) -> Self {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait Hit {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb>;
}
