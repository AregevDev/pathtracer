use crate::material::{Diffuse, Material};
use crate::ray::Ray;
use crate::vector::Vector3;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: Rc<RefCell<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: 0.0,
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: Rc::new(RefCell::new(Diffuse::new(Vector3::new(0.0, 0.0, 0.0)))),
        }
    }
}

pub trait Hit {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> (bool, HitRecord);
}
