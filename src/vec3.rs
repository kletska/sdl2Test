use rand::prelude::*;

use std::ops;

const MAXCOLOR: f32 = 255.99;
const EPS: f32 = 0.001;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }
    
    pub fn length(self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn unit_vector(v: Vec3) -> Vec3 { 
        v / v.length()
    }
    
    pub fn dotProd(a: Vec3, b: Vec3) -> f32 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
    }

    pub fn cross_prod(a: Vec3, b : Vec3) -> Vec3 {
        Vec3(a.1 * b.2 - a.2 * b.1, 
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0
        )
    }

    pub fn to_color(self) -> u32 {
        let ir: u32 = (self.0 * MAXCOLOR) as u32;
        let ig: u32 = (self.1 * MAXCOLOR) as u32;
        let ib: u32 = (self.2 * MAXCOLOR) as u32;
        return (ib * 256u32 + ig) * 256u32 + ir
    }

    pub fn sqr_length(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn near_zero(&self) -> bool {
        self.0.abs() < EPS && self.1.abs() < EPS && self.2.abs() < EPS
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::default() - self
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs) 
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::default();
    loop {
        p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3::new(1.0, 1.0, 1.0);
        if p.sqr_length() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::default();
    loop {
        p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.sqr_length() < 1.0 {
            return p;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec_add() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0), Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vec_mul() {
        assert_eq!(Vec3::new(7.0, 8.0, 9.0) * 10.0, Vec3::new(70.0, 80.0, 90.0));
    }

    #[test]
    fn test_vec_div() {
        assert_eq!(Vec3::new(3.0, 6.0, 9.0) / 3.0, Vec3::new(1.0, 2.0, 3.0));
    }
}
