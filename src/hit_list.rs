use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;

pub struct HitList {
    pub hits: Vec<Box<dyn Hit>>,
}

impl HitList {
    pub fn new() -> Self {
        HitList { hits: vec![] }
    }

    pub fn add<H: Hit + 'static>(&mut self, hit: H) {
        self.hits.push(Box::new(hit));
    }
}

impl Hit for HitList {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;

        for hittable in self.hits.iter() {
            if let Some(rec) = hittable.hit(r, t_min, t_max) {
                match hit {
                    None => hit = Some(rec),
                    Some(prev) => {
                        if rec.t < prev.t {
                            hit = Some(rec)
                        }
                    }
                }
            }
        }

        hit
    }
}
