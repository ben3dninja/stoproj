use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0., 0., 0.);
    pub const X: Vec3 = Vec3::new(1., 0., 0.);
    pub const Y: Vec3 = Vec3::new(0., 1., 0.);
    pub const Z: Vec3 = Vec3::new(0., 0., 1.);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl From<Vec<f32>> for Vec3 {
    fn from(v: Vec<f32>) -> Self {
        Vec3::new(v[0], v[1], v[2])
    }
}

impl From<&str> for Vec3 {
    fn from(s: &str) -> Self {
        let s = if s.starts_with('v') {
            s.strip_prefix("v ").unwrap().to_string()
        } else {
            s.to_string()
        };
        s.split(' ')
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<f32>>()
            .into()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v {} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        assert_eq!(Vec3::from("v 1.0 2.3 -4.0"), Vec3::new(1., 2.3, -4.))
    }
}
