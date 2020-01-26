use crate::aabb::surrounding_box;
use crate::aabb::Aabb;
use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use rand::Rng;
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

pub struct BvhTree<'a> {
    root: usize,
    nodes: Vec<BvhNode<'a>>,
}

impl<'a> BvhTree<'a> {
    pub fn new(models: &'a mut [Box<dyn Hit>]) -> BvhTree {
        let mut tree = BvhTree {
            nodes: Vec::new(),
            root: 0,
        };
        tree.root = tree.build(models);

        tree
    }

    fn build(&mut self, models: &'a mut [Box<dyn Hit>]) -> usize {
        let axis = rand::thread_rng().gen_range::<i32, i32, i32>(0, 3);
        match axis {
            0 => models.sort_by(|a, b| box_x_compare(a, b)),
            1 => models.sort_by(|a, b| box_y_compare(a, b)),
            2 => models.sort_by(|a, b| box_z_compare(a, b)),
            _ => unreachable!(),
        }

        let left: usize;
        let right: usize;

        match models.len() {
            1 => return self.add_leaf(&models[0]),
            2 => {
                left = self.add_leaf(&models[0]);
                right = self.add_leaf(&models[1]);
            }
            _ => {
                let half_len = models.len() / 2;
                let (hit_left, hit_right) = models.split_at_mut(half_len);

                left = self.build(hit_left);
                right = self.build(hit_right);
            }
        }

        if let Some(left_box) = self.nodes[left].bounding {
            if let Some(right_box) = self.nodes[right].bounding {
                return self.add_node(surrounding_box(left_box, right_box), left, right);
            }
        }

        panic!("No bounding box found")
    }

    fn add_leaf(&mut self, h: &'a Box<dyn Hit>) -> usize {
        let next = self.nodes.len();
        self.nodes.push(BvhNode {
            left: None,
            right: None,
            hittable: Some(h),
            bounding: h.bounding_box(0.0, 0.0),
        });

        next
    }

    fn add_node(&mut self, bounding: Aabb, left: usize, right: usize) -> usize {
        let next = self.nodes.len();
        self.nodes.push(BvhNode {
            left: Some(left),
            right: Some(right),
            hittable: None,
            bounding: Some(bounding),
        });

        next
    }

    fn hit_node(&self, id: usize, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let node = &self.nodes[id];

        if node.bounding.is_none()
            || node.bounding.is_some() && node.bounding.unwrap().hit(ray, t_min, t_max)
        {
            match node.hittable {
                Some(h) => return h.hit(ray, t_min, t_max),
                None => {}
            }
        }

        let mut hit_left: Option<HitRecord> = None;
        let mut hit_right: Option<HitRecord> = None;

        if let Some(left_index) = node.left {
            hit_left = self.hit_node(left_index, ray, t_min, t_max);
        }

        if let Some(right_index) = node.right {
            hit_right = self.hit_node(right_index, ray, t_min, t_max);
        }

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
}

impl<'a> Hit for BvhTree<'a> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hit_node(self.root, ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        self.nodes[self.root].bounding
    }
}

pub struct BvhNode<'a> {
    left: Option<usize>,
    right: Option<usize>,
    hittable: Option<&'a Box<dyn Hit>>,
    bounding: Option<Aabb>,
}
