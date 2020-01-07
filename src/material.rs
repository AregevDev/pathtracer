use crate::hit::HitRecord;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vector::Vector3;
use std::fmt::Debug;
use rand::Rng;

fn schick(cos: f32, ref_index: f32) -> f32 {
    let r0 = ((1.0 - ref_index) / (1.0 + ref_index)).powi(2);

    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

pub trait Material {
    fn scatter(&self, ray_in: Ray, record: &HitRecord) -> (bool, Vector3, Ray);
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
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
        let new_fuzz = fuzz.min(1.0);

        Metal {
            albedo,
            fuzz: new_fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, record: &HitRecord) -> (bool, Vector3, Ray) {
        let reflected = ray_in.direction.reflect(&record.normal);
        let attenuation = self.albedo;
        let ray_out = Ray::new(record.p, random_in_unit_sphere() * self.fuzz + reflected);

        (
            ray_out.direction.dot(&record.normal) > 0.0,
            attenuation,
            ray_out,
        )
    }
}

pub struct Dielectric {
    ref_index: f32,
}

impl Dielectric {
    pub fn new(ref_index: f32) -> Self {
        Dielectric {
            ref_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, record: &HitRecord) -> (bool, Vector3, Ray) {
        let reflected = ray_in.direction.reflect(&record.normal);
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let ray_out;

        let outward_normal;
        let ni_over_nt;

        if ray_in.direction.dot(&record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt = self.ref_index;
        }
        else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.ref_index;
        }

        let refracted = ray_in.direction.refract(&outward_normal, ni_over_nt);
        if refracted.0 {
            ray_out = Ray::new(record.p, refracted.1);
        } else {
            ray_out = Ray::new(record.p, reflected);
            return (false, attenuation, ray_out)
        }

        (true, attenuation, ray_out)
    }
}
