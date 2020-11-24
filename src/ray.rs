use crate::vec3::Vec3;

pub struct Ray {
    base: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray{ base: a, dir: b } 
    }

    pub fn origin(&self) -> Vec3 {
        self.base
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn point_at_parametr(&self, t: f32) -> Vec3 {
        self.base + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ray_origin() {
    }

    #[test]
    fn test_ray_direction() {
    }

    #[test]
    fn test_ray_point_at_parametr() {
    }
}
