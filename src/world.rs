use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;

pub struct World {
    hits: Vec<Box<dyn Hit>>,
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
}
