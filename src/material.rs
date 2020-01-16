use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::vector::Vector3;
use crate::random_in_unit_sphere;

pub trait Material {
    // Return an optional scattered ray and attenuation
    fn scatter(&self, ray_in: Ray, record: &HitRecord) -> Option<(Ray, Vector3)>;
}

pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: Ray, record: &HitRecord) -> Option<(Ray, Vector3)> {
        let target = record.p + record.normal + random_in_unit_sphere();

        Some((Ray::new(record.p, target - record.p), self.albedo))
    }
}
