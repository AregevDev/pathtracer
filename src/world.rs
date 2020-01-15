use crate::hit::Hit;
use crate::record::HitRecord;
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
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest = t_max;

        for i in 0..self.hits.len() {
            if self.hits[i].hit(ray, t_min, closest, &mut temp_record) {
                hit_anything = true;
                closest = temp_record.t;
                *record = temp_record;
            }
        }

        hit_anything
    }
}
