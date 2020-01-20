use crate::ray::Ray;
use crate::vector::Vector3;
use std::cmp;

pub fn min(a: f32, b: f32) -> f32 {
    return if a < b { a } else { b };
}

pub fn max(a: f32, b: f32) -> f32 {
    return if a > b { a } else { b };
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Aabb {
    pub min: Vector3,
    pub max: Vector3,
}

impl Aabb {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Aabb { min, max }
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            let min = if t0 > t_min { t0 } else { t_min };
            let max = if t1 < t_min { t1 } else { t_max };

            if max <= min {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = Vector3::new(
        box0.min.x.min(box1.min.x),
        box0.min.y.min(box1.min.y),
        box0.min.z.min(box1.min.z),
    );

    let big = Vector3::new(
        box0.max.x.max(box1.max.x),
        box0.max.y.max(box1.max.y),
        box0.max.z.max(box1.max.z),
    );

    Aabb::new(small, big)
}
