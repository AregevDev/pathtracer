use crate::hit::{Hit, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector3;
use std::rc::Rc;

pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let result: Option<HitRecord> = None;

        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                let record = HitRecord::new(temp, p, normal, self.material.clone());
                return Some(record);
            }

            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                let record = HitRecord::new(temp, p, normal, self.material.clone());
                return Some(record);
            }
        }

        result
    }
}
