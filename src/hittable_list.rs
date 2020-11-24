use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        None
    }
}
