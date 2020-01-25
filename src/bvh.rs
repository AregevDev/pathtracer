use crate::aabb::surrounding_box;
use crate::aabb::Aabb;
use crate::hit::{Hit, HitRecord};
use crate::random_float;
use crate::ray::Ray;
use std::cmp::Ordering;

macro_rules! box_compare {
    ($f:ident, $a:ident) => {
        fn $f(a: &Box<dyn Hit>, b: &Box<dyn Hit>) -> Ordering {
            let box_left = a.bounding_box(0.0, 0.0).unwrap();
            let box_right = b.bounding_box(0.0, 0.0).unwrap();

            return if box_left.min.$a - box_right.min.$a < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
    };
}

box_compare!(box_x_compare, x);
box_compare!(box_y_compare, y);
box_compare!(box_z_compare, z);

pub struct NullBvhNode {
    aabb: Aabb,
}

impl Hit for NullBvhNode {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(self.aabb)
    }
}

pub struct BvhNode {
    left: Box<dyn Hit>,
    right: Box<dyn Hit>,
    aabb: Aabb,
}

impl BvhNode {
    pub fn new(mut hits: Vec<Box<dyn Hit>>, time0: f32, time1: f32) -> Self {
        let axis = (3.0 * random_float()) as i32;
        if axis == 0 {
            hits.sort_by(box_x_compare);
        } else if axis == 1 {
            hits.sort_by(box_y_compare);
        } else {
            hits.sort_by(box_z_compare);
        }

        let left: Box<dyn Hit>;
        let right: Box<dyn Hit>;

        let l = hits.len();
        if l == 1 {
            left = hits.remove(0);
            right = Box::new(NullBvhNode {
                aabb: left.bounding_box(time0, time1).unwrap(),
            })
        } else if l == 2 {
            left = hits.remove(0);
            right = hits.remove(1);
        } else {
            let rest = hits.split_off(l / 2);
            left = Box::new(BvhNode::new(hits, time0, time1));
            right = Box::new(BvhNode::new(rest, time0, time1));
        }

        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();

        BvhNode {
            left,
            right,
            aabb: surrounding_box(box_left, box_right),
        }
    }
}

impl Hit for BvhNode {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.aabb.hit(ray, t_min, t_max) {
            let hit_left = self.left.hit(ray, t_min, t_max);
            let hit_right = self.right.hit(ray, t_min, t_max);

            return match (hit_left, hit_right) {
                (Some(left), Some(right)) => {
                    if left.t < right.t {
                        Some(left)
                    } else {
                        Some(right)
                    }
                }
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),
                _ => None,
            };
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(self.aabb)
    }
}
