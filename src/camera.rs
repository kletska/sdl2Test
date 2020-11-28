use crate::vec3::{Vec3, random_in_unit_disk};
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3, 
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f32,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,

}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, theta: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera { 
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross_prod(vup, w));
        let v = Vec3::cross_prod(w, u);

        Camera {
            lower_left_corner: origin - focus_dist * (half_width * u + half_height * v + w),
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin,
            lens_radius: aperture / 2.0,
            u,
            v,
            w
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.0 + self.v * rd.1;
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
    }
}

