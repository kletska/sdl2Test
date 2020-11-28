use rand::prelude::*;

use crate::vec3::{Vec3, random_in_unit_sphere};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};


#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ref_idx: f32 }, 
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            &Material::Lambertian { albedo } => {
                let mut target = rec.normal + random_in_unit_sphere();
                if target.near_zero() {
                    target = rec.normal;
                }
                //*scattered = Ray::new(rec.p, rec.normal);
                *scattered = Ray::new(rec.p, target);
                *attenuation = albedo;
                true
            }
            &Material::Metal { albedo, mut fuzz } => {
                if fuzz > 1.0 {
                    fuzz = 1.0;
                }
                let reflect = reflect(Vec3::unit_vector(ray_in.direction()), rec.normal);
                *scattered = Ray::new(rec.p, reflect + fuzz * random_in_unit_sphere()); 
                *attenuation = albedo;
                Vec3::dotProd(scattered.direction(), rec.normal) > 0.0
            }
            &Material::Dielectric { ref_idx } => {
                let mut outward_normal = Vec3::default();
                let reflected = reflect(ray_in.direction(), rec.normal);
                let mut ni_over_nt = 0.0;
                *attenuation = Vec3::new(1.0, 1.0, 1.0);
                let mut refracted = Vec3::default();

                let mut reflect_prob = 0.0;
                let mut cosine = 0.0;
               
                if Vec3::dotProd(ray_in.direction(), rec.normal) > 0.0 {
                    outward_normal = -rec.normal;
                    ni_over_nt = ref_idx;
                    cosine = ref_idx * Vec3::dotProd(ray_in.direction(), rec.normal) / ray_in.direction().length();
                } else {
                    outward_normal = rec.normal;
                    ni_over_nt = 1.0 / ref_idx;
                    cosine = -Vec3::dotProd(ray_in.direction(), rec.normal) / ray_in.direction().length();
                }
                
                if refract(ray_in.direction(), outward_normal, ni_over_nt, &mut refracted) {
                    reflect_prob = schlick(cosine, ref_idx);
                } else {
                    reflect_prob = 1.0;
                }
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() < reflect_prob {
                    *scattered = Ray::new(rec.p, reflected);
                } else {
                    *scattered = Ray::new(rec.p, refracted);
                }
                true
            }
        }
    }
}

impl Default for Material {
    fn default() -> Material {
        Material::Lambertian { albedo: Vec3::default() }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32{
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dotProd(uv, n);
    let d = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if d > 0.0 {
        *refracted = ni_over_nt * (uv - n * dt) - n * d.sqrt();
        true 
    } else {
        false
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dotProd(v, n) * n
}
