use crate::aabb::{Aabb, min, surrounding_box};
use crate::hit::{Hit, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector3;
use std::rc::Rc;

pub struct MovingSphere {
    center0: Vector3,
    center1: Vector3,
    radius: f32,
    time0: f32,
    time1: f32,
    material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vector3,
        center1: Vector3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Rc<dyn Material>,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            radius,
            material,
            time0,
            time1,
        }
    }

    pub fn center(&self, time: f32) -> Vector3 {
        (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
            + self.center0
    }
}

impl Hit for MovingSphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let result: Option<HitRecord> = None;

        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center(ray.time)) / self.radius;
                let record = HitRecord::new(temp, p, normal, self.material.clone());
                return Some(record);
            }

            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center(ray.time)) / self.radius;
                let record = HitRecord::new(temp, p, normal, self.material.clone());
                return Some(record);
            }
        }

        result
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(t0) - Vector3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vector3::new(self.radius, self.radius, self.radius),
        );

        let box1 = Aabb::new(
            self.center(t1) - Vector3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vector3::new(self.radius, self.radius, self.radius),
        );

        Some(surrounding_box(box0, box1))
    }
}
