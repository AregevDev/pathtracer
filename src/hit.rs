use crate::ray::Ray;
use crate::record::HitRecord;
use crate::vector::Vector3;

pub trait Hit {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vector3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                return true;
            }

            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                return true;
            }
        }

        false
    }
}
