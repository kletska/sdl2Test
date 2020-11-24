use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(a: Vec3, b: f32) -> Sphere {
        Sphere{ center: a, radius: b}
    }
}

impl Hittable for Sphere {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        None         
    }
}
