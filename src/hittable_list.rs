use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList{ list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        for object in &self.list {
             match result {
                 None => result = object.hit(r, t_min, t_max),
                 Some(res) => match object.hit(r, t_min, res.t) {
                        None => (),
                        ather => result = ather
                    }
             }
        } 
        result
    }
}
