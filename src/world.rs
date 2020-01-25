use crate::aabb::surrounding_box;
use crate::aabb::Aabb;
use crate::camera::Camera;
use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;

pub struct World {
    pub hits: Vec<Box<dyn Hit>>,
}

impl World {
    pub fn new() -> Self {
        World { hits: Vec::new() }
    }

    pub fn add<H>(&mut self, hit: H)
    where
        H: Hit + 'static,
    {
        self.hits.push(Box::new(hit));
    }
}

impl Hit for World {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest = t_max;

        for h in self.hits.iter() {
            if let Some(record) = h.hit(ray, t_min, closest) {
                closest = record.t;
                result = Some(record)
            }
        }

        result
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        let mut result: Option<Aabb> = None;
        let mut temp = Aabb::default();

        if self.hits.len() < 1 {
            return None;
        }

        let bb1 = self.hits[0].bounding_box(t0, t1);
        if let Some(bb1) = bb1 {
            let mut bb = bb1;
            for h in self.hits.iter().skip(1) {
                if let Some(temp_box) = h.bounding_box(t0, t1) {
                    bb = surrounding_box(bb, temp_box);
                } else {
                    return None;
                }
            }

            return Some(bb);
        }

        None
    }
}
