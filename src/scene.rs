use crate::bvh::BvhTree;
use crate::hit::Hit;

pub struct SceneSettings {
    pub width: i32,
    pub height: i32,
    pub spp: i32,
    pub max_bounce: i32,
}

pub struct Scene<'a> {
    pub settings: SceneSettings,
    pub bvh: BvhTree<'a>,
}

impl<'a> Scene<'a> {
    pub fn new(models: &'a mut Vec<Box<dyn Hit>>, settings: SceneSettings) -> Self {
        Scene {
            settings,
            bvh: BvhTree::new(models),
        }
    }
}
