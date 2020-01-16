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

pub struct Metal {
    albedo: Vector3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f32) -> Self {
        Metal { albedo, fuzz: fuzz.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, record: &HitRecord) -> Option<(Ray, Vector3)> {
        let reflected = ray_in.direction.normalize().reflect(record.normal);
        let scattered = Ray::new(record.p, reflected + random_in_unit_sphere() * self.fuzz);

        return if scattered.direction.dot(record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
