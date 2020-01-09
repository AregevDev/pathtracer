use crate::hit::HitRecord;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vector::Vector3;
use std::fmt::Debug;

#[derive(Debug, Default, Copy, Clone)]
pub struct Scatter {
    pub color: Vector3,
    pub ray: Option<Ray>,
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vector3 },
    Metal { albedo: Vector3, fuzz: f32 },
    Dielectric { refractive_index: f32 },
}

impl Material {
    pub fn lambertian(albedo: Vector3) -> Self {
        Material::Lambertian { albedo }
    }

    pub fn metal(albedo: Vector3, fuzz: f32) -> Self {
        Material::Metal { albedo, fuzz }
    }

    pub fn dielectric(refractive_index: f32) -> Self {
        Material::Dielectric { refractive_index }
    }

    pub fn scatter(&self, ray_in: Ray, record: &HitRecord) -> Option<Scatter> {
        match *self {
            Material::Lambertian { albedo } => {
                let mut scattered = Scatter::default();
                let target = record.p + record.normal + random_in_unit_sphere();

                scattered.color = albedo;
                scattered.ray = Some(Ray::new(record.p, target - record.p));

                Some(scattered)
            }
            Material::Metal { albedo, fuzz } => {
                let mut scattered = Scatter::default();
                let reflected = ray_in.direction.normalize().reflect(record.normal);
                let new_fuzz = fuzz.min(1.0);

                scattered.color = albedo;
                scattered.ray = Some(Ray::new(
                    record.p,
                    reflected + new_fuzz * random_in_unit_sphere(),
                ));

                if scattered.ray.unwrap().direction.dot(record.normal) > 0.0 {
                    Some(scattered)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Vector3::default(),
        }
    }
}
