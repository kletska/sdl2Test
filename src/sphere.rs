use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = Vec3::dotProd(r.direction(), r.direction());
        let b = Vec3::dotProd(oc, r.direction());
        let c = Vec3::dotProd(oc, oc) - self.radius * self.radius;
        let d = b * b - a * c;
        if d > 0.0 {
            let mut temp = (-b - d.sqrt()) / (a);
            let mut temp_vec = r.point_at_parametr(temp);
            if t_min < temp && temp < t_max {
                Some(HitRecord::new(temp, temp_vec, (temp_vec - self.center) / self.radius, self.material))
            } else {
                temp = (-b + d.sqrt()) / (a);
                temp_vec = r.point_at_parametr(temp);
                if t_min < temp && temp < t_max {
                    Some(HitRecord::new(temp, temp_vec, (temp_vec - self.center) / self.radius, self.material))
                } else {
                    None    
                }
            }
        } else {
            None
        }
    }
}
