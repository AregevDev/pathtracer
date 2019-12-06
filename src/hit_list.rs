use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;

#[derive(Default)]
pub struct HitList {
    pub hits: Vec<Box<dyn Hit>>,
}

impl HitList {
    pub fn new() -> Self {
        HitList::default()
    }

    pub fn add(&mut self, hit: Box<dyn Hit>) {
        self.hits.push(hit);
    }
}

impl Hit for HitList {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest = t_max;
        for hit in self.hits.iter() {
            let (hit, record) = hit.hit(r, t_min, closest);

            if hit {
                hit_anything = true;
                closest = record.t;
                temp_rec = record;
            }
        }

        (hit_anything, temp_rec)
    }
}
