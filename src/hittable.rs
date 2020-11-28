use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: Material) -> HitRecord {
        HitRecord{ t, p, normal, material }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>; 
}
