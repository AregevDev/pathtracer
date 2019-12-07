use crate::hit::HitRecord;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vector::Vector3;
use std::fmt::Debug;

pub trait Material {
    fn scatter(&self, ray_in: Ray, record: &HitRecord) -> (bool, Vector3, Ray);
}

#[derive(Debug)]
pub struct Diffuse {
    albedo: Vector3,
}

impl Diffuse {
    pub fn new(albedo: Vector3) -> Self {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray_in: Ray, record: &HitRecord) -> (bool, Vector3, Ray) {
        let target = record.p + record.normal + random_in_unit_sphere();
        let attenuation = self.albedo;
        let ray_out = Ray::new(record.p, target - record.p);

        (true, attenuation, ray_out)
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Vector3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f32) -> Self {
        let new_fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };

        Metal {
            albedo,
            fuzz: new_fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, record: &HitRecord) -> (bool, Vector3, Ray) {
        let reflected = ray_in.direction.normalize().reflect(&record.normal);
        let attenuation = self.albedo;
        let ray_out = Ray::new(record.p, random_in_unit_sphere() * self.fuzz + reflected);

        (
            ray_out.direction.dot(&record.normal) > 0.0,
            attenuation,
            ray_out,
        )
    }
}
