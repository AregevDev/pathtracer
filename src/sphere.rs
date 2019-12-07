use crate::hit::{Hit, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector3;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub material: Rc<RefCell<dyn Material>>,
}

impl Sphere {
    pub fn new<M: Material + 'static>(center: Vector3, radius: f32, material: M) -> Self {
        Sphere {
            center,
            radius,
            material: Rc::new(RefCell::new(material)),
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - (self.radius * self.radius);

        let discriminant = b * b - a * c;
        let mut record = HitRecord::new();

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                record.material = self.material.clone();
                return (true, record);
            }
        }

        (false, record)
    }
}
