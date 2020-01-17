use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector3;
use crate::{random_float, random_in_unit_sphere};

pub fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

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
    fn scatter(&self, _ray_in: Ray, record: &HitRecord) -> Option<(Ray, Vector3)> {
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
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
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
        };
    }
}

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, record: &HitRecord) -> Option<(Ray, Vector3)> {
        let reflected = ray_in.direction.normalize().reflect(record.normal);
        let scattered;
        let outward_normal;
        let ni_over_nt;

        let reflect_prob;
        let cosine;

        if ray_in.direction.dot(record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt = self.refractive_index;
            cosine = ray_in.direction.dot(record.normal) / ray_in.direction.length()
                * self.refractive_index;
        } else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -ray_in.direction.dot(record.normal) / ray_in.direction.length();
        }

        let refracted = ray_in.direction.refract(outward_normal, ni_over_nt);
        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        if ray_in.direction.refract(outward_normal, ni_over_nt) != Vector3::default() {
            reflect_prob = schlick(cosine, self.refractive_index);
        } else {
            reflect_prob = 1.0;
        }

        if random_float() < reflect_prob {
            scattered = Ray::new(record.p, reflected);
        } else {
            scattered = Ray::new(record.p, refracted);
        }

        Some((scattered, attenuation))
    }
}
